extern crate rush_sys;

use std::os::raw::c_void;
use rush_sys::*;

pub trait App {
    fn on_startup(&mut self){}
    fn on_update(&mut self){}
    fn on_shutdown(&mut self){}
}

#[no_mangle]
pub extern "C" fn on_startup(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_startup();
}

#[no_mangle]
pub extern "C" fn on_update(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_update();
}

#[no_mangle]
pub extern "C" fn on_shutdown(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_shutdown();
}

pub fn run(app : Box<dyn App>) -> i32 {

    let mut app_cfg = rush_app_config::new();

    let user_data_box : Box<Box<dyn App>> = Box::new(app);
    app_cfg.user_data = Box::into_raw(user_data_box) as *mut _;
    app_cfg.on_startup = Some(on_startup);
    app_cfg.on_update = Some(on_update);
    app_cfg.on_shutdown = Some(on_shutdown);

    let err_code = unsafe { rush_sys::rush_platform_main(&app_cfg) };

    err_code
}
