#![allow(dead_code)]

extern crate rush;
use rush::*;

extern crate rush_sys;
use rush_sys::*;

struct HelloTriangleApp {
    vb: GfxBuffer,
    cb: GfxBuffer,
    technique: rush_gfx_technique,
}

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
    fn default() ->Self{
        Constants {
            transform3d: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
            transform2d: [
                1.0, 1.0, 0.0, 0.0,
            ],
            color: [
                1.0, 1.0, 1.0, 1.0,
            ],
        }
    }
}

impl HelloTriangleApp {
    fn new(_platform: &mut Platform) -> HelloTriangleApp {
        let vs = GfxVertexShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_2D_VS)
        });
        let ps = GfxPixelShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_PLAIN_PS)
        });

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

        let vf = unsafe { rush_gfx_create_vertex_format(vf_desc.as_ptr(), vf_desc.len() as u32) };

        let descriptor_sets = [rush_gfx_descriptor_set_desc {
            constant_buffers: 1,
            ..Default::default()
        }];

        let technique_desc = rush_gfx_technique_desc {
            vs: vs.native,
            ps: ps.native,
            vf: vf,
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

        let cb_data = [
            Constants::default()
        ];

        HelloTriangleApp {
            vb: GfxBuffer::new_with_data(&GfxBufferDesc {
                flags: GfxBufferFlags::VERTEX,
                format: GfxFormat::UNKNOWN,
                stride: std::mem::size_of::<Vertex>() as u32,
                count: vb_data.len() as u32,
                host_visible: false,
            }, vb_data.as_ptr()),
            cb: GfxBuffer::new_with_data(&GfxBufferDesc {
                flags: GfxBufferFlags::CONSTANT,
                format: GfxFormat::UNKNOWN,
                stride: std::mem::size_of::<Constants>() as u32,
                count: 1,
                host_visible: false,
            }, cb_data.as_ptr()),
            technique: unsafe { rush_gfx_create_technique(&technique_desc) },
        }
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let ctx = &mut platform.gfx_context;

        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA::black_alpha(1.0),
            }],
            flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
            ..Default::default()
        };

        ctx.begin_pass(&pass_desc);
        ctx.set_technique(self.technique);
        ctx.set_vertex_buffer(0, &self.vb);
        ctx.set_constant_buffer(0, &self.cb, 0);
        ctx.set_primitive_type(GfxPrimitiveType::TriangleList);
        ctx.draw(0, 3);        
        ctx.end_pass();
    }
}

// todo: find a way to move the bootstrap into the core library
struct BootstrapApp {
    app: Option<HelloTriangleApp>,
}

impl App for BootstrapApp {
    fn on_startup(&mut self, platform: &mut Platform) {
        self.app = Some(HelloTriangleApp::new(platform));
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let app: &mut HelloTriangleApp = self.app.as_mut().unwrap();
        app.on_update(platform);
    }
    fn on_shutdown(&mut self, _platform: &mut Platform) {
        self.app = None;
    }
}

fn main() {
    let app = Box::new(BootstrapApp { app: None });
    rush::run(app);
}
