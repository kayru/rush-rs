extern crate rush_rs;
use rush_rs::*;

struct HelloWorldApp {
	counter : u32,
}

impl App for HelloWorldApp {
	fn on_startup(&mut self, _platform: &mut Platform) {
		println!("HelloWorldApp::startup()");
	}
	fn on_update(&mut self, platform: &mut Platform) {
		let ctx = &mut platform.gfx_context;

		let pass_desc = GfxPassDesc {
			color: vec![
				GfxColorTarget{
					target: None,
					clear_color: ColorRGBA{r:0.1, g:0.2, b:0.3, a:1.0}}
			],
			flags: GfxPassFlags::CLEAR_COLOR_DEPTH_STENCIL,
			..Default::default()
		};

		ctx.begin_pass(&pass_desc);
		ctx.end_pass();

		if self.counter % 25 == 0 {
			println!("HelloWorldApp::update() {}", self.counter);
		}
		self.counter += 1;
	}
	fn on_shutdown(&mut self, _platform: &mut Platform) {
		println!("HelloWorldApp::shutdown()");
	}
}

fn main() {
	let app = Box::new(HelloWorldApp{
		counter: 0
	});
	rush_rs::run(app);
}
