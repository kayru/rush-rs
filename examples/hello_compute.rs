extern crate rush_rs;
use rush_rs::*;

extern crate rush_sys;
use rush_sys::*;
extern crate shaderc;
use std::ffi::CString;

use std::time::Instant;

static SOURCE_TEXT: &str = r#"
struct Constants
{
	float4 params;
};

cbuffer constantBuffer0 : register(b0)
{
	Constants globals;
};

RWTexture2D<float4> Output : register(u1);

float noise(float2 t, float s)
{
    return frac(sin(s+dot(t.xy, float2(12.9898, 78.233))) * 43758.5453);
}

float bouncy(float2 v, float2 fragCoord, float4 params)
{
	float2 cp = v * params.z;
	float2 cp_wrap = float2(uint2(cp) / uint2(params.xy));
	cp = fmod(cp, params.xy);
	cp = lerp(cp, params.xy - cp, fmod(cp_wrap, float2(2.0, 2.0)));
	return 25.0 / (1.0+length(cp - fragCoord.xy));
}

[numthreads(8,8,1)]
void main(uint2 dtid : SV_DISPATCHTHREADID)
{
    float2 fragCoord = (float2)dtid;
	float3 res = float3(0, 0, 0);
	res += float3(1.0, 0.3, 0.2) * bouncy(float2(211, 312), fragCoord.xy, globals.params);
	res += float3(0.3, 1.0, 0.2) * bouncy(float2(312, 210), fragCoord.xy, globals.params);
	res += float3(0.2, 0.3, 1.0) * bouncy(float2(331, 130), fragCoord.xy, globals.params);
	float2 p = fragCoord.xy;
	float dither = (noise(p, globals.params.z) - 0.5) / 64.0;
    Output[dtid] = float4(res + dither, 1);
}
"#;

#[repr(C)]
struct Constants {
    params: [f32; 4],
}

fn main() {
    let mut app = AppContext::new();
    let entry_name = CString::new("main").unwrap();
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut compile_options = shaderc::CompileOptions::new().unwrap();
    compile_options.set_source_language(shaderc::SourceLanguage::HLSL);
    let binary_result = compiler
        .compile_into_spirv(
            SOURCE_TEXT,
            shaderc::ShaderKind::Compute,
            "shader.hlsl",
            entry_name.to_str().unwrap(),
            Some(&compile_options),
        )
        .unwrap();
    let cs_source = rush_gfx_shader_source {
        type_: RUSH_GFX_SHADER_SOURCE_SPV,
        entry: entry_name.as_ptr(),
        data: binary_result.as_binary().as_ptr() as *const ::std::os::raw::c_void,
        size_bytes: binary_result.len() as u32,
    };
    let cs = GfxComputeShader::new_with_source(&cs_source);
    let descriptor_sets = [rush_gfx_descriptor_set_desc {
        constant_buffers: 1,
        rw_images: 1,
        ..Default::default()
    }];
    let technique_desc = rush_gfx_technique_desc {
        cs: cs.native,
        bindings: rush_gfx_shader_bindings_desc {
            descriptor_sets: descriptor_sets.as_ptr(),
            descriptor_set_count: descriptor_sets.len() as u32,
            use_default_descriptor_set: true,
        },
        ..Default::default()
    };
    let cb_data = [Constants {
        params: [0.0, 0.0, 0.0, 0.0],
    }];
    let frame_size = (app.cfg.width, app.cfg.height);
    let cb = GfxBuffer::new_with_data(
        &GfxBufferDesc {
            flags: GfxBufferFlags::CONSTANT | GfxBufferFlags::TRANSIENT,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<Constants>() as u32,
            count: 1,
            host_visible: false,
        },
        cb_data.as_ptr(),
    );
    let mut prim = GfxPrimitiveBatch::new();
    let frame = GfxTexture::new(&GfxTextureDesc {
        width: frame_size.0 as u32,
        height: frame_size.1 as u32,
        depth: 1,
        mips: 1,
        samples: 1,
        format: GfxFormat::RGBA8_UNORM,
        texture_type: GfxTextureType::Tex2D,
        usage: GfxUsageFlags::STORAGE_IMAGE | GfxUsageFlags::SHADER_RESOURCE,
    });
    let technique = GfxTechnique::new(&technique_desc);
    let time_start = Instant::now();
    AppContext::run(|| {
        let ctx = &mut app.gfx_context;
        let elapsed_time = (Instant::now() - time_start).as_secs_f32();
        let cb_data = [Constants {
            params: [
                frame_size.0 as f32, // frame width
                frame_size.1 as f32, // frame height
                elapsed_time,
                0.0,
            ],
        }];
        ctx.update_buffer_from_array(&cb, cb_data.as_ptr(), 1);
        ctx.set_technique(&technique);
        ctx.set_constant_buffer(0, &cb, 0);
        ctx.set_storate_image(0, &frame);
        ctx.dispatch(frame_size.0 as u32 / 8, frame_size.1 as u32 / 8, 1);
        ctx.add_image_barrier(&frame, RUSH_GFX_RESOURCE_STATE_SHADER_READ);
        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA::black_alpha(1.0),
            }],
            flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
            ..Default::default()
        };
        ctx.begin_pass(&pass_desc);
        prim.begin_2d((frame_size.0 as f32, frame_size.1 as f32));
        prim.set_texture(ctx, frame.native, GfxBatchSampler::Linear);
        prim.draw_textured_rect_2d(
            ctx,
            (0.0, 0.0, frame_size.0 as f32, frame_size.1 as f32),
            (0.0, 0.0, 1.0, 1.0),
            ColorRGBA8::white(),
        );
        prim.end_2d(ctx);
        ctx.end_pass();
    });
}
