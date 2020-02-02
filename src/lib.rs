#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

extern crate rush_sys;

use std::os::raw::c_void;
use rush_sys::*;

pub struct GfxDevice {
    native  : *mut rush_gfx_device,
}

pub struct GfxContext {
    native : *mut rush_gfx_context,
}

pub struct GfxTexture {
    native : rush_gfx_texture,
}

impl GfxTexture {
    fn invalid() -> Self {
        GfxTexture { native: rush_gfx_texture{handle: 0}}
    }
}

impl Default for GfxTexture {
    fn default() -> Self {
        GfxTexture::invalid()
    }
}

pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for ColorRGBA {
    fn default() -> Self {
        ColorRGBA {r: 0.0, g: 0.0, b: 0.0, a: 0.0}
    }
}

bitflags! {
    pub struct GfxPassFlags: u32 {
        const NONE = rush_gfx_pass_flags_RUSH_GFX_PASS_NONE;
        const CLEAR_COLOR = rush_gfx_pass_flags_RUSH_GFX_PASS_CLEAR_COLOR;
        const CLEAR_DEPTH_STENCIL = rush_gfx_pass_flags_RUSH_GFX_PASS_CLEAR_DEPTH_STENCIL;
        const DISCARD_COLOR = rush_gfx_pass_flags_RUSH_GFX_PASS_DISCARD_COLOR;
        const CLEAR_COLOR_DEPTH_STENCIL = Self::CLEAR_COLOR.bits | Self::CLEAR_DEPTH_STENCIL.bits;
    }
}

impl Default for GfxPassFlags {
    fn default() -> Self {
        GfxPassFlags::NONE
    }
}

pub struct GfxColorTarget {
    pub target : Option<GfxTexture>,
    pub clear_color : ColorRGBA,
}

pub struct GfxDepthTarget {
    pub target : Option<GfxTexture>,
    pub clear_depth : f32,
    pub clear_stencil : u8,
}

pub struct GfxPassDesc {
    pub color : Vec<GfxColorTarget>,
	pub depth : GfxDepthTarget,
	pub flags : GfxPassFlags,
}

impl Default for GfxPassDesc {
    fn default() -> Self {
        GfxPassDesc {
            color : Vec::new(),
            depth: GfxDepthTarget { target: None, clear_depth: 1.0, clear_stencil: 0xFF },
            flags: GfxPassFlags::default(),
        }
    }
}

impl GfxContext {
    pub fn begin_pass(&mut self, desc : &GfxPassDesc) {
        let mut color_targets : Vec<rush_gfx_color_target> = Vec::with_capacity(desc.color.len());
        for c in &desc.color {
            let native = rush_gfx_color_target {
                clear_color : rush_color_rgba {
                    r: c.clear_color.r,
                    g: c.clear_color.g,
                    b: c.clear_color.b,
                    a: c.clear_color.a,
                },
                target : match &c.target {
                    Some(texture) => texture.native,
                    None => GfxTexture::invalid().native
                },
            };
            color_targets.push(native);
        }
        let depth_target = rush_gfx_depth_target {
            target : match &desc.depth.target {
                Some(texture) => texture.native,
                None => GfxTexture::invalid().native
            },
            clear_depth: desc.depth.clear_depth,
            clear_stencil: desc.depth.clear_stencil,
        };

        unsafe {
            rush_gfx_begin_pass(self.native, 
                color_targets.len() as u32,
                color_targets.as_ptr(),
                &depth_target,
                desc.flags.bits
            )
        };
    }

    pub fn end_pass(&mut self) {
        unsafe { rush_gfx_end_pass(self.native) };
    }
}

pub struct Platform {
    pub gfx_device  : GfxDevice,
    pub gfx_context : GfxContext,
}

impl Platform {
    pub fn new() -> Platform {
        Platform {
            gfx_device  : GfxDevice  { native : unsafe { rush_platform_get_device() } },
            gfx_context : GfxContext { native : unsafe { rush_platform_get_context() } },
        }
    }
}

pub trait App {
    fn on_startup  (&mut self, _platform: &mut Platform){}
    fn on_update   (&mut self, _platform: &mut Platform){}
    fn on_shutdown (&mut self, _platform: &mut Platform){}
}

#[no_mangle]
pub extern "C" fn on_startup(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_startup(&mut Platform::new());
}

#[no_mangle]
pub extern "C" fn on_update(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_update(&mut Platform::new());
}

#[no_mangle]
pub extern "C" fn on_shutdown(user_data: *mut c_void) {
    let app_box : &mut Box<dyn App> = unsafe {std::mem::transmute(user_data)};
    app_box.on_shutdown(&mut Platform::new());
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
