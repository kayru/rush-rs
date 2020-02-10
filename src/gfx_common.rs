extern crate rush_sys;
use rush_sys::*;

pub struct GfxTexture {
    pub native: rush_gfx_texture,
}

impl GfxTexture {
    pub fn invalid() -> Self {
        GfxTexture {
            native: rush_gfx_texture { handle: 0 },
        }
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
        ColorRGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
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
    pub target: Option<GfxTexture>,
    pub clear_color: ColorRGBA,
}

pub struct GfxDepthTarget {
    pub target: Option<GfxTexture>,
    pub clear_depth: f32,
    pub clear_stencil: u8,
}

pub struct GfxPassDesc {
    pub color: Vec<GfxColorTarget>,
    pub depth: GfxDepthTarget,
    pub flags: GfxPassFlags,
}

impl Default for GfxPassDesc {
    fn default() -> Self {
        GfxPassDesc {
            color: Vec::new(),
            depth: GfxDepthTarget {
                target: None,
                clear_depth: 1.0,
                clear_stencil: 0xFF,
            },
            flags: GfxPassFlags::default(),
        }
    }
}
