extern crate rush;
use rush::*;

struct GfxPrimitivesApp {

}

impl App for GfxPrimitivesApp {
    fn on_startup(&mut self, _platform: &mut Platform) {

    }
	fn on_update(&mut self, _platform: &mut Platform) {

	}
}

fn main() {
	let app = Box::new(GfxPrimitivesApp{});
	rush::run(app);
}
