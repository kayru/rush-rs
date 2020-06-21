extern crate rush_rs;
use rush_rs::*;

extern crate rush_sys;
use rush_sys::*;

#[repr(C)]
struct Vertex {
    pos: [f32; 3],
    tex: [f32; 2],
    col: [u8; 4],
}

#[repr(C)]
struct Constants {
    transform3d: [f32; 16], // mat4
    transform2d: [f32; 4],  // xy: scale, zw: bias
    color: [f32; 4],
}

impl Default for Constants {
    fn default() -> Self {
        Constants {
            #[rustfmt::skip]
            transform3d: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
            transform2d: [1.0, 1.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

fn main() {
    let mut app = AppContext::new();
    let vs = GfxVertexShader::new_embedded(GfxEmbeddedVertexShader::Primitive2D);
    let ps = GfxPixelShader::new_embedded(GfxEmbeddedPixelShader::PrimitivePlain);
    let vf_desc = [
        rush_gfx_vertex_element {
            semantic: RUSH_GFX_VERTEX_SEMANTIC_POSITION,
            index: 0,
            format: RUSH_GFX_FORMAT_RGB32_FLOAT,
            stream: 0,
        },
        rush_gfx_vertex_element {
            semantic: RUSH_GFX_VERTEX_SEMANTIC_TEXCOORD,
            index: 0,
            format: RUSH_GFX_FORMAT_RG32_FLOAT,
            stream: 0,
        },
        rush_gfx_vertex_element {
            semantic: RUSH_GFX_VERTEX_SEMANTIC_COLOR,
            index: 0,
            format: RUSH_GFX_FORMAT_RGBA8_UNORM,
            stream: 0,
        },
    ];
    let vf = GfxVertexFormat::new(&vf_desc);
    let descriptor_sets = [rush_gfx_descriptor_set_desc {
        constant_buffers: 1,
        ..Default::default()
    }];
    let technique_desc = rush_gfx_technique_desc {
        vs: vs.native,
        ps: ps.native,
        vf: vf.native,
        bindings: rush_gfx_shader_bindings_desc {
            descriptor_sets: descriptor_sets.as_ptr(),
            descriptor_set_count: descriptor_sets.len() as u32,
            use_default_descriptor_set: true,
        },
        ..Default::default()
    };
    let vb_data = [
        Vertex {
            pos: [-0.5, -0.5, 0.0],
            tex: [0.0, 0.0],
            col: [0xFF, 0, 0, 0xFF],
        },
        Vertex {
            pos: [0.0, 0.5, 0.0],
            tex: [0.0, 0.0],
            col: [0, 0xFF, 0, 0xFF],
        },
        Vertex {
            pos: [0.5, -0.5, 0.0],
            tex: [0.0, 0.0],
            col: [0, 0, 0xFF, 0xFF],
        },
    ];
    let cb_data = [Constants::default()];
    let vb = GfxBuffer::new_with_data(
        &GfxBufferDesc {
            flags: GfxBufferFlags::VERTEX,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<Vertex>() as u32,
            count: vb_data.len() as u32,
            host_visible: false,
        },
        vb_data.as_ptr(),
    );
    let cb = GfxBuffer::new_with_data(
        &GfxBufferDesc {
            flags: GfxBufferFlags::CONSTANT,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<Constants>() as u32,
            count: 1,
            host_visible: false,
        },
        cb_data.as_ptr(),
    );
    let technique = GfxTechnique::new(&technique_desc);
    AppContext::run(|| {
        let ctx = &mut app.gfx_context;
        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA::black_alpha(1.0),
            }],
            flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
            ..Default::default()
        };
        ctx.begin_pass(&pass_desc);
        ctx.set_technique(&technique);
        ctx.set_vertex_buffer(0, &vb);
        ctx.set_constant_buffer(0, &cb, 0);
        ctx.set_primitive_type(GfxPrimitiveType::TriangleList);
        ctx.draw(0, 3);
        ctx.end_pass();
    });
}
