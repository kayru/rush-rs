extern crate rush_rs;
use rush_rs::*;

struct HelloPrimitivesApp {
    prim: GfxPrimitiveBatch,
    font: GfxBitmapFont,
}

impl HelloPrimitivesApp {
    fn new(_platform: &mut Platform) -> HelloPrimitivesApp {
        HelloPrimitivesApp {
            prim: GfxPrimitiveBatch::new(),
            font: GfxBitmapFont::new_embedded(false, (0,0))
        }
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let ctx = &mut platform.gfx_context;
        let prim = &mut self.prim;

        let pass_desc = GfxPassDesc {
            color: vec![GfxColorTarget {
                target: None,
                clear_color: ColorRGBA {
                    r: 11.0 / 255.0,
                    g: 22.0 / 255.0,
                    b: 33.0 / 255.0,
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

            prim.draw_line_2d(
                ctx,
                (100.0, 100.0, 100.0, 200.0),
                splat2!(ColorRGBA8::red()),
            );

            prim.draw_line_2d(
                ctx,
                (110.0, 100.0, 110.0, 200.0),
                splat2!(ColorRGBA8::green()),
            );

            prim.draw_line_2d(
                ctx,
                (120.0, 100.0, 120.0, 200.0),
                splat2!(ColorRGBA8::blue()),
            );

            prim.draw_rect_2d(
                ctx,
                (130.0, 100.0, 150.0, 200.0),
                ColorRGBA8::new(128, 128, 255, 255),
            );
            prim.flush(ctx);

            prim.draw_tri_2d(
                ctx,
                (160.0, 100.0),
                (200.0, 100.0),
                (200.0, 200.0),
                splat3!(ColorRGBA8::new(255, 128, 128, 255)),
            );

            prim.end_2d(ctx);
        }

        {
            prim.begin_2d_scale_bias((1.0, 1.0), (0.0, 0.0));

            prim.draw_line_2d(ctx, (0.0, 0.0, 1.0, 0.0), splat2!(ColorRGBA8::red()));
            prim.draw_line_2d(ctx, (0.0, 0.0, 0.0, 1.0), splat2!(ColorRGBA8::green()));

            prim.draw_tri_2d(
                ctx,
                (-0.5, -0.5),
                (-0.5, 0.0),
                (0.0, -0.5),
                (ColorRGBA8::red(), ColorRGBA8::green(), ColorRGBA8::blue()),
            );

            prim.end_2d(ctx);
        }

        // Text rendering
        {
            prim.begin_2d(window_size);

            GfxBitmapFontRenderer::new(ctx, prim, &self.font)
                .set_position((10.0, 10.0))
                .set_color(ColorRGBA8::red())
                .print("Hello rust world!\n")
                .set_color(ColorRGBA8::green())
                .print("This is line 2.\n")
                .set_color(ColorRGBA8::white())
                .print("the quick brown fox jumps over the lazy dog\n")
                .print("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG\n")
                .print("0 1 2 3 4 5 6 7 8 9\n");

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
    rush_rs::run(app);
}
