extern crate rush_sys;
use rush_sys::*;

pub struct GfxDevice {
    native: *mut rush_gfx_device,
}

impl GfxDevice {
    pub fn new(native: *mut rush_gfx_device) -> Self {
        GfxDevice { native: native }
    }
}
