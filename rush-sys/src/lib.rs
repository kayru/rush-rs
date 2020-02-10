#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate const_cstr;

include!("rush_ffi.rs");

const_cstr! {
    DEFAULT_APP_NAME = "RushApp";
}

impl rush_app_config {
    pub fn new() -> rush_app_config {
        rush_app_config {
            name: DEFAULT_APP_NAME.as_ptr(),
            vsync: 1,
            width: 640,
            height: 480,

            max_width: 0,
            max_height: 0,

            fullscreen: false,
            resizable: false,
            debug: false,
            warp: false,
            minimize_latency: false,

            argc: 0,
            argv: std::ptr::null_mut(),

            user_data: std::ptr::null_mut(),

            on_startup: None,
            on_update: None,
            on_shutdown: None,
        }
    }
}
