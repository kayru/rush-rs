#![allow(dead_code, unused_macros, unused_variables)]

extern crate rush;
use rush::*;

macro_rules! splat2 {
    ($v:expr) => {{
        let v = $v;
        (v, v)
    }}
}

macro_rules! splat3 {
    ($v:expr) => {{
        let v = $v;
        (v, v, v)
    }}
}

struct HelloPrimitivesApp {
    prim: GfxPrimitiveBatch,
}

impl HelloPrimitivesApp {
    fn new(_platform: &mut Platform) -> HelloPrimitivesApp {
        HelloPrimitivesApp {
            prim: GfxPrimitiveBatch::new(),
        }
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let ctx = &mut platform.gfx_context;
        let prim = &mut self.prim;

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

        let window_size = (640.0, 480.0); // todo: query window size

        ctx.begin_pass(&pass_desc);

        // Basic shape rendering
        {
            prim.begin_2d(window_size);

            prim.draw_line_2d(ctx, (100.0, 100.0, 100.0, 200.0), splat2!(ColorRGBA8::red()));
            prim.draw_line_2d(ctx, (110.0, 100.0, 110.0, 200.0), splat2!(ColorRGBA8::green()));
            prim.draw_line_2d(ctx, (120.0, 100.0, 120.0, 200.0), splat2!(ColorRGBA8::blue()));

            prim.end_2d(ctx);
        }

        ctx.end_pass();
    }
}

// todo: find a way to move the bootstrap into the core library
struct BootstrapApp {
    app: Option<HelloPrimitivesApp>,
}

impl App for BootstrapApp {
    fn on_startup(&mut self, platform: &mut Platform) {
        self.app = Some(HelloPrimitivesApp::new(platform));
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let app: &mut HelloPrimitivesApp = self.app.as_mut().unwrap();
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
