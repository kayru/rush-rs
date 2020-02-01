extern crate winit;

pub struct Window {
    pub winit_events_loop: winit::EventsLoop,
    pub winit_window: winit::Window,
    pub running: bool,
}

impl Window {
    pub fn new() -> Window {
        let window_title = "Untitled".to_string();
        let winit_events_loop = winit::EventsLoop::new();
        let winit_window = winit::WindowBuilder::new()
            .with_title(window_title)
            .with_dimensions((640, 480).into())
            .build(&winit_events_loop).unwrap();
        let result = Window {
            winit_events_loop: winit_events_loop,
            winit_window: winit_window,
            running: true,
        };
        return result;
    }
}
