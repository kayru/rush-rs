#![allow(dead_code, unused_macros, unused_variables, unused_assignments)]

extern crate rush_rs;
use rush_rs::*;

extern crate rush_sys;
use rush_sys::*;

struct HelloWindowApp {
    pub event_listener: *mut rush_window_event_listener, // todo: safe high level wrapper
}

impl HelloWindowApp {
    fn new(_platform: &mut Platform) -> HelloWindowApp {
        HelloWindowApp {
            event_listener: unsafe {
                rush_window_create_listener(
                    rush_platform_get_main_window(),
                    RUSH_WINDOW_EVENT_MASK_ALL,
                )
            },
        }
    }
    fn on_update(&mut self, platform: &mut Platform) {
        // todo: safe high level wrapper
        let count = unsafe { rush_window_event_listener_count(self.event_listener) as u32 };
        let mut events: Vec<rush_window_event> = Vec::with_capacity(count as usize);
        let received_count = unsafe {
            rush_window_event_listener_receive(self.event_listener, count, events.as_mut_ptr())
        };
        unsafe { events.set_len(received_count as usize) };

        for event in events {
            match event.event_type {
                RUSH_WINDOW_EVENT_TYPE_KEY_DOWN => println!("key_down: key={} modifiers={}", event.key, event.modifiers),
                RUSH_WINDOW_EVENT_TYPE_KEY_UP => println!("key_up: key={}", event.key),
                RUSH_WINDOW_EVENT_TYPE_RESIZE => println!("resize: width={} height={}", event.width, event.height),
                RUSH_WINDOW_EVENT_TYPE_CHAR => println!("char: character={}", event.character),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_DOWN => println!("mouse_down: button={} pos.x={} pos.y={} doubleclick={}", event.button, event.pos[0], event.pos[1], event.double_click as u32),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_UP => println!("mouse_up: button={} pos.x={} pos.y={} doubleclick={}", event.button, event.pos[0], event.pos[1], event.double_click as u32),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_MOVE => println!("mouse_move: pos.x={} pos.y={}", event.pos[0], event.pos[1]),
                RUSH_WINDOW_EVENT_TYPE_SCROLL => println!("scroll: scroll.x={} scroll.y={}", event.scroll[0], event.scroll[1]),
                _ => println!("Unknown event type"),
            }
        }
    }
}

// todo: find a way to move the bootstrap into the core library
struct BootstrapApp {
    app: Option<HelloWindowApp>,
}

impl App for BootstrapApp {
    fn on_startup(&mut self, platform: &mut Platform) {
        self.app = Some(HelloWindowApp::new(platform));
    }
    fn on_update(&mut self, platform: &mut Platform) {
        let app: &mut HelloWindowApp = self.app.as_mut().unwrap();
        app.on_update(platform);
    }
    fn on_shutdown(&mut self, _platform: &mut Platform) {
        let app: &mut HelloWindowApp = self.app.as_mut().unwrap();
        unsafe {
            rush_window_destroy_listener(app.event_listener);
        }
        self.app = None;
    }
}

fn main() {
    let app = Box::new(BootstrapApp { app: None });
    rush_rs::run(app);
}
