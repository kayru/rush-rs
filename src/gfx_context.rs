extern crate rush_sys;
use crate::gfx_common::*;
use rush_sys::*;

pub struct GfxContext {
    native: *mut rush_gfx_context,
}

impl GfxContext {
    pub fn new(native: *mut rush_gfx_context) -> Self {
        GfxContext { native: native }
    }
    pub fn begin_pass(&mut self, desc: &GfxPassDesc) {
        let mut color_targets: Vec<rush_gfx_color_target> = Vec::with_capacity(desc.color.len());
        for c in &desc.color {
            let native = rush_gfx_color_target {
                clear_color: rush_color_rgba {
                    r: c.clear_color.r,
                    g: c.clear_color.g,
                    b: c.clear_color.b,
                    a: c.clear_color.a,
                },
                target: match &c.target {
                    Some(texture) => texture.native,
                    None => GfxTexture::invalid().native,
                },
            };
            color_targets.push(native);
        }
        let depth_target = rush_gfx_depth_target {
            target: match &desc.depth.target {
                Some(texture) => texture.native,
                None => GfxTexture::invalid().native,
            },
            clear_depth: desc.depth.clear_depth,
            clear_stencil: desc.depth.clear_stencil,
        };
        unsafe {
            rush_gfx_begin_pass(
                self.native,
                color_targets.len() as u32,
                color_targets.as_ptr(),
                &depth_target,
                desc.flags.bits(),
            )
        };
    }

    pub fn end_pass(&mut self) {
        unsafe { rush_gfx_end_pass(self.native) };
    }

    pub fn draw(&mut self, first_vertex: u32, vertex_count: u32) {
        unsafe { rush_gfx_draw(self.native, first_vertex, vertex_count) };
    }
}
