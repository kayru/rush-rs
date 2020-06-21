extern crate rush_rs;
use rush_rs::*;

extern crate rush_sys;
use rush_sys::*;

fn main() {
    let _app = AppContext::new();
    let event_listener = unsafe {
        // todo: safe high level wrapper
        rush_window_create_listener(rush_platform_get_main_window(), RUSH_WINDOW_EVENT_MASK_ALL)
    };
    AppContext::run(|| {
        let count = unsafe { rush_window_event_listener_count(event_listener) as u32 };
        let mut events: Vec<rush_window_event> = Vec::with_capacity(count as usize);
        let received_count = unsafe {
            rush_window_event_listener_receive(event_listener, count, events.as_mut_ptr())
        };
        unsafe { events.set_len(received_count as usize) };
        for event in events {
            match event.event_type {
                RUSH_WINDOW_EVENT_TYPE_KEY_DOWN => {
                    println!("key_down: key={} modifiers={}", event.key, event.modifiers)
                }
                RUSH_WINDOW_EVENT_TYPE_KEY_UP => println!("key_up: key={}", event.key),
                RUSH_WINDOW_EVENT_TYPE_RESIZE => {
                    println!("resize: width={} height={}", event.width, event.height)
                }
                RUSH_WINDOW_EVENT_TYPE_CHAR => println!("char: character={}", event.character),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_DOWN => println!(
                    "mouse_down: button={} pos.x={} pos.y={} doubleclick={}",
                    event.button, event.pos[0], event.pos[1], event.double_click as u32
                ),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_UP => println!(
                    "mouse_up: button={} pos.x={} pos.y={} doubleclick={}",
                    event.button, event.pos[0], event.pos[1], event.double_click as u32
                ),
                RUSH_WINDOW_EVENT_TYPE_MOUSE_MOVE => {
                    println!("mouse_move: pos.x={} pos.y={}", event.pos[0], event.pos[1])
                }
                RUSH_WINDOW_EVENT_TYPE_SCROLL => println!(
                    "scroll: scroll.x={} scroll.y={}",
                    event.scroll[0], event.scroll[1]
                ),
                _ => println!("Unknown event type"),
            }
        }
    });
}
