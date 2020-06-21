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

pub mod gfx_bitmap_font;
pub use self::gfx_bitmap_font::*;

#[macro_export]
macro_rules! splat2 {
    ($v:expr) => {{
        let v = $v;
        (v, v)
    }};
}

#[macro_export]
macro_rules! splat3 {
    ($v:expr) => {{
        let v = $v;
        (v, v, v)
    }};
}

#[macro_export]
macro_rules! splat4 {
    ($v:expr) => {{
        let v = $v;
        (v, v, v, v)
    }};
}

#[cfg(debug_assertions)]
fn should_use_debug_layer() -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn should_use_debug_layer() -> bool {
    false
}

pub struct AppContext {
    pub cfg: rush_app_config,
    pub gfx_context: GfxContext,
}

extern "C" fn ffi_closure<F>(closure: *mut c_void)
where
    F: FnMut(),
{
    let opt_closure = closure as *mut Option<F>;
    unsafe {
        (*opt_closure).as_mut().unwrap()();
    }
}

impl AppContext {
    pub fn new() -> Self {
        let mut app_cfg = rush_app_config::new();
        app_cfg.debug = should_use_debug_layer();
        unsafe {
            rush_sys::rush_platform_startup(&app_cfg);
        }
        AppContext {
            cfg: app_cfg,
            gfx_context: GfxContext::new(unsafe { rush_platform_get_context() }),
        }
    }
    pub fn run<F>(frame_fn: F)
    where
        F: FnMut(),
    {
        unsafe {
            let user_data = &frame_fn as *const _ as *mut c_void;
            rush_platform_run(Some(ffi_closure::<F>), user_data);
        }
    }
}

impl Drop for AppContext {
    fn drop(&mut self) {
        unsafe { rush_sys::rush_platform_shutdown() };
    }
}
