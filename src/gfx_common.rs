#![allow(non_camel_case_types)]

extern crate rush_sys;
use rush_sys::*;

#[derive(Copy, Clone)]
pub enum GfxFormat {
    UNKNOWN = rush_gfx_format_RUSH_GFX_FORMAT_UNKNOWN as isize,
    D24_UNORM_S8_UINT = rush_gfx_format_RUSH_GFX_FORMAT_D24_UNORM_S8_UINT as isize,
    D24_UNORM_X8 = rush_gfx_format_RUSH_GFX_FORMAT_D24_UNORM_X8 as isize,
    D32_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_D32_FLOAT as isize,
    D32_FLOAT_S8_UINT = rush_gfx_format_RUSH_GFX_FORMAT_D32_FLOAT_S8_UINT as isize,
    R8_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_R8_UNORM as isize,
    R16_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_R16_FLOAT as isize,
    R16_UINT = rush_gfx_format_RUSH_GFX_FORMAT_R16_UINT as isize,
    R32_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_R32_FLOAT as isize,
    R32_UINT = rush_gfx_format_RUSH_GFX_FORMAT_R32_UINT as isize,
    RG8_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_RG8_UNORM as isize,
    RG16_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_RG16_FLOAT as isize,
    RG32_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_RG32_FLOAT as isize,
    RGB32_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_RGB32_FLOAT as isize,
    RGB8_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_RGB8_UNORM as isize,
    RGBA16_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_RGBA16_FLOAT as isize,
    RGBA16_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_RGBA16_UNORM as isize,
    RGBA32_FLOAT = rush_gfx_format_RUSH_GFX_FORMAT_RGBA32_FLOAT as isize,
    RGBA8_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_RGBA8_UNORM as isize,
    RGBA8_SRGB = rush_gfx_format_RUSH_GFX_FORMAT_RGBA8_SRGB as isize,
    BGRA8_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BGRA8_UNORM as isize,
    BGRA8_SRGB = rush_gfx_format_RUSH_GFX_FORMAT_BGRA8_SRGB as isize,
    BC1_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BC1_UNORM as isize,
    BC1_UNORM_SRGB = rush_gfx_format_RUSH_GFX_FORMAT_BC1_UNORM_SRGB as isize,
    BC3_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BC3_UNORM as isize,
    BC3_UNORM_SRGB = rush_gfx_format_RUSH_GFX_FORMAT_BC3_UNORM_SRGB as isize,
    BC4_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BC4_UNORM as isize,
    BC5_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BC5_UNORM as isize,
    BC6H_UFLOAT = rush_gfx_format_RUSH_GFX_FORMAT_BC6H_UFLOAT as isize,
    BC6H_SFLOAT = rush_gfx_format_RUSH_GFX_FORMAT_BC6H_SFLOAT as isize,
    BC7_UNORM = rush_gfx_format_RUSH_GFX_FORMAT_BC7_UNORM as isize,
    BC7_UNORM_SRGB = rush_gfx_format_RUSH_GFX_FORMAT_BC7_UNORM_SRGB as isize,
}

pub struct GfxVertexShader {
    pub native: rush_gfx_vertex_shader,
}

impl GfxVertexShader {
    pub fn new_with_source(source: &rush_gfx_shader_source) -> Self {
        GfxVertexShader {
            native: unsafe {
                rush_gfx_create_vertex_shader(source)
            }
        }
    }
}

pub struct GfxPixelShader {
    pub native: rush_gfx_pixel_shader,
}

impl GfxPixelShader {
    pub fn new_with_source(source: &rush_gfx_shader_source) -> Self {
        GfxPixelShader {
            native: unsafe {
                rush_gfx_create_pixel_shader(source)
            }
        }
    }
}

pub struct GfxTexture {
    pub native: rush_gfx_texture,
}

impl Default for GfxTexture {
    fn default() -> Self {
        GfxTexture {
            native: rush_gfx_texture { handle: 0 },
        }
    }
}

pub struct GfxBuffer {
    pub native: rush_gfx_buffer,
}

impl Default for GfxBuffer {
    fn default() -> Self {
        GfxBuffer {
            native: rush_gfx_buffer { handle: 0 },
        }
    }
}

impl GfxBuffer {
    pub fn new(desc: &GfxBufferDesc) -> GfxBuffer {
        let native_desc = rush_gfx_buffer_desc::from(desc);
        GfxBuffer {
            native: unsafe { rush_gfx_create_buffer(&native_desc, std::ptr::null()) },
        }
    }
    pub fn new_with_data<T>(desc: &GfxBufferDesc, data: *const T) -> GfxBuffer {
        let native_desc = rush_gfx_buffer_desc::from(desc);
        GfxBuffer {
            native: unsafe {
                rush_gfx_create_buffer(&native_desc, data as *const ::std::os::raw::c_void)
            },
        }
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

bitflags! {
    pub struct GfxBufferFlags: u32 {
        const NONE = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_NONE;
        const VERTEX = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_VERTEX;
        const INDEX = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_INDEX;
        const CONSTANT = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_CONSTANT;
        const STORAGE = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_STORAGE;
        const TEXEL = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_TEXEL;
        const INDIRECT_ARGS = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_INDIRECT_ARGS;
        const RAYTRACING = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_RAYTRACING;
        const TRANSIENT = rush_gfx_buffer_flags_RUSH_GFX_BUFFER_FLAG_TRANSIENT;
    }
}

impl Default for GfxBufferFlags {
    fn default() -> Self {
        GfxBufferFlags::NONE
    }
}

pub struct GfxBufferDesc {
    pub flags: GfxBufferFlags,
    pub format: GfxFormat,
    pub stride: u32,
    pub count: u32,
    pub host_visible: bool,
}

impl From<&GfxBufferDesc> for rush_gfx_buffer_desc {
    fn from(desc: &GfxBufferDesc) -> Self {
        rush_gfx_buffer_desc {
            flags: desc.flags.bits(),
            format: desc.format as rush_gfx_format,
            stride: desc.stride,
            count: desc.count,
            host_visible: desc.host_visible,
        }
    }
}
