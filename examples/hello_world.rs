extern crate rush;

use rush::App;

struct HelloWorldApp
{
	counter : u32,
}

impl App for HelloWorldApp
{
	fn on_startup(&mut self) {
		println!("HelloWorldApp::startup()");
	}

	fn on_update(&mut self)	{
		if self.counter % 25 == 0 {
			println!("HelloWorldApp::update() {}", self.counter);
		}
		self.counter += 1;
	}

	fn on_shutdown(&mut self) {
		println!("HelloWorldApp::shutdown()");
	}
}

fn main() {
	let app = Box::new(HelloWorldApp{
		counter: 0
	});
	rush::run(app);
}
