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

struct HelloComputeApp {
    cb: GfxBuffer,
    frame_size: (u32, u32),
    frame: GfxTexture,
    technique: GfxTechnique,
    prim: GfxPrimitiveBatch,
    time_start: Instant,
}

#[repr(C)]
struct Constants {
    params: [f32; 4],
}

impl HelloComputeApp {
    fn new(_platform: &mut Platform) -> HelloComputeApp {
        let descriptor_sets = [rush_gfx_descriptor_set_desc {
            constant_buffers: 1,
            rw_images: 1,
            ..Default::default()
        }];

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

        let frame_size = (640, 480);
        HelloComputeApp {
            cb: GfxBuffer::new_with_data(
                &GfxBufferDesc {
                    flags: GfxBufferFlags::CONSTANT | GfxBufferFlags::TRANSIENT,
                    format: GfxFormat::UNKNOWN,
                    stride: std::mem::size_of::<Constants>() as u32,
                    count: 1,
                    host_visible: false,
                },
                cb_data.as_ptr(),
            ),
            prim: GfxPrimitiveBatch::new(),
            frame_size,
            frame: GfxTexture::new(&GfxTextureDesc {
                width: frame_size.0,
                height: frame_size.1,
                depth: 1,
                mips: 1,
                samples: 1,
                format: GfxFormat::RGBA8_UNORM,
                texture_type: GfxTextureType::Tex2D,
                usage: GfxUsageFlags::STORAGE_IMAGE | GfxUsageFlags::SHADER_RESOURCE,
            }),
            technique: GfxTechnique::new(&technique_desc),
            time_start: Instant::now(),
        }
    }
    
    fn on_update(&mut self, platform: &mut Platform) {
        let ctx = &mut platform.gfx_context;

        let elapsed_time = (Instant::now() - self.time_start).as_secs_f32();
        let cb_data = [Constants {
            params: [
                self.frame_size.0 as f32, // frame width
                self.frame_size.1 as f32, // frame height
                elapsed_time,
                0.0,
            ],
        }];

        ctx.update_buffer_from_array(&self.cb, cb_data.as_ptr(), 1);

        ctx.set_technique(&self.technique);
        ctx.set_constant_buffer(0, &self.cb, 0);
        ctx.set_storate_image(0, &self.frame);
        ctx.dispatch(self.frame_size.0 / 8, self.frame_size.1 / 8, 1);

        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA::black_alpha(1.0),
            }],
            flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
            ..Default::default()
        };

        ctx.begin_pass(&pass_desc);
        let prim = &mut self.prim;
        prim.begin_2d((self.frame_size.0 as f32, self.frame_size.1 as f32));
        prim.set_texture(ctx, self.frame.native, GfxBatchSampler::Linear);
        prim.draw_textured_rect_2d(
            ctx,
            (0.0, 0.0, self.frame_size.0 as f32, self.frame_size.1 as f32),
            (0.0, 0.0, 1.0, 1.0),
            ColorRGBA8::white(),
        );
        prim.end_2d(ctx);
        ctx.end_pass();
    }
}

// todo: find a way to move the bootstrap into the core library
struct BootstrapApp {
    app: Option<HelloComputeApp>,
}

impl App for BootstrapApp {
    fn on_startup(&mut self, platform: &mut Platform) {
        self.app = Some(HelloComputeApp::new(platform));
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let app: &mut HelloComputeApp = self.app.as_mut().unwrap();
        app.on_update(platform);
    }
    fn on_shutdown(&mut self, _platform: &mut Platform) {
        self.app = None;
    }
}

fn main() {
    let app = Box::new(BootstrapApp { app: None });
    rush_rs::run(app);
}
