extern crate rush_sys;

use std::os::raw::c_void;
use rush_sys::*;

#[no_mangle]
pub extern "C" fn on_startup(_user_data: *mut c_void) {
    println!("on_startup (rust)");
}

#[no_mangle]
pub extern "C" fn on_update(_user_data: *mut c_void) {
    println!("on_update (rust)");
}

#[no_mangle]
pub extern "C" fn on_shutdown(_user_data: *mut c_void) {
    println!("on_shutdown (rust)");
}

pub fn run() -> i32 {

    let mut app_cfg = rush_app_config::new();

    app_cfg.on_startup = Some(on_startup);
    app_cfg.on_update = Some(on_update);
    app_cfg.on_shutdown = Some(on_shutdown);

    unsafe { rush_sys::rush_platform_main(&app_cfg) }
}
