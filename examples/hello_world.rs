extern crate rush_rs;
use rush_rs::*;

fn main() {
	println!("HelloWorldApp::startup");
	let mut app = AppContext::new();
	let mut counter = 0;
	AppContext::run(||{
		let ctx = &mut app.gfx_context;
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
		if counter % 25 == 0 {
			println!("HelloWorldApp::update {}", counter);
		}
		counter += 1;
	});
	println!("HelloWorldApp::shutdown");
}
