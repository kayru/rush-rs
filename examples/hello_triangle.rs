#![allow(dead_code)]

extern crate rush;
use rush::*;

struct HelloTriangleApp {
	vb : GfxBuffer,
}

impl HelloTriangleApp {
	fn new(_platform: &mut Platform) -> HelloTriangleApp {
        let vb_data : Vec<f32> = vec![
            0.0, 0.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, 0.0, 0.0,
        ];
		let vb_desc = GfxBufferDesc {
            flags: GfxBufferFlags::VERTEX,
            format: GfxFormat::RGB32_FLOAT,
            stride: 12,
            count: 3,
            host_visible: false
        };
		HelloTriangleApp {
			vb : GfxBuffer::new_with_data(&vb_desc, vb_data.as_ptr())
		}
	}
    fn on_update(&mut self, _platform: &mut Platform) {
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
