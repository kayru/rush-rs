#![allow(dead_code)]

extern crate rush;
use rush::*;

extern crate rush_sys;
use rush_sys::*;

struct HelloTriangleApp {
    vb: GfxBuffer,
}

#[repr(C)]
struct Vertex {
    pos: [f32; 3],
    tex: [f32; 2],
    col: [u8; 4],
}

impl HelloTriangleApp {
    fn new(_platform: &mut Platform) -> HelloTriangleApp {
        let _vs = GfxVertexShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(
                RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_2D_VS,
            )
        });
        let _ps = GfxPixelShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(
                RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_PLAIN_PS,
            )
        });

        let vb_data: Vec<Vertex> = vec![
            Vertex {
                pos: [0.0, 0.0, 0.0],
                tex: [0.0, 0.0],
                col: [0xFF, 0, 0, 0xFF],
            },
            Vertex {
                pos: [1.0, 1.0, 0.0],
                tex: [0.0, 0.0],
                col: [0, 0xFF, 0, 0xFF],
            },
            Vertex {
                pos: [1.0, 0.0, 0.0],
                tex: [0.0, 0.0],
                col: [0, 0, 0xFF, 0xFF],
            },
        ];
        let vb_desc = GfxBufferDesc {
            flags: GfxBufferFlags::VERTEX,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<Vertex>() as u32,
            count: vb_data.len() as u32,
            host_visible: false,
        };
        HelloTriangleApp {
            vb: GfxBuffer::new_with_data(&vb_desc, vb_data.as_ptr()),
        }
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let ctx = &mut platform.gfx_context;

        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                },
            }],
            flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
            ..Default::default()
        };

        ctx.begin_pass(&pass_desc);

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
