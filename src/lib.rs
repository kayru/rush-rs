#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

extern crate rush_sys;

use rush_sys::*;
use std::os::raw::c_void;

pub mod gfx_common;
pub use self::gfx_common::*;

pub mod gfx_context;
pub use self::gfx_context::*;

pub mod gfx_primitive_batch;
pub use self::gfx_primitive_batch::*;

pub struct Platform {
    pub gfx_context: GfxContext,
}

impl Platform {
    pub fn new() -> Platform {
        Platform {
            gfx_context: GfxContext::new(unsafe { rush_platform_get_context() }),
        }
    }
}

pub trait App {
    fn on_startup(&mut self, _platform: &mut Platform) {}
    fn on_update(&mut self, _platform: &mut Platform) {}
    fn on_shutdown(&mut self, _platform: &mut Platform) {}
}

#[no_mangle]
pub extern "C" fn on_startup(user_data: *mut c_void) {
    let app_box: &mut Box<dyn App> = unsafe { std::mem::transmute(user_data) };
    app_box.on_startup(&mut Platform::new());
}

#[no_mangle]
pub extern "C" fn on_update(user_data: *mut c_void) {
    let app_box: &mut Box<dyn App> = unsafe { std::mem::transmute(user_data) };
    app_box.on_update(&mut Platform::new());
}

#[no_mangle]
pub extern "C" fn on_shutdown(user_data: *mut c_void) {
    let app_box: &mut Box<dyn App> = unsafe { std::mem::transmute(user_data) };
    app_box.on_shutdown(&mut Platform::new());
}

pub fn run(app: Box<dyn App>) -> i32 {
    let mut app_cfg = rush_app_config::new();

    let user_data_box: Box<Box<dyn App>> = Box::new(app);
    app_cfg.user_data = Box::into_raw(user_data_box) as *mut _;
    app_cfg.on_startup = Some(on_startup);
    app_cfg.on_update = Some(on_update);
    app_cfg.on_shutdown = Some(on_shutdown);
    app_cfg.debug = true;

    let err_code = unsafe { rush_sys::rush_platform_main(&app_cfg) };

    err_code
}
