#![allow(non_camel_case_types)]

extern crate rush_sys;
use rush_sys::*;

#[derive(Copy, Clone)]
pub enum GfxFormat {
    UNKNOWN = RUSH_GFX_FORMAT_UNKNOWN as isize,
    D24_UNORM_S8_UINT = RUSH_GFX_FORMAT_D24_UNORM_S8_UINT as isize,
    D24_UNORM_X8 = RUSH_GFX_FORMAT_D24_UNORM_X8 as isize,
    D32_FLOAT = RUSH_GFX_FORMAT_D32_FLOAT as isize,
    D32_FLOAT_S8_UINT = RUSH_GFX_FORMAT_D32_FLOAT_S8_UINT as isize,
    R8_UNORM = RUSH_GFX_FORMAT_R8_UNORM as isize,
    R16_FLOAT = RUSH_GFX_FORMAT_R16_FLOAT as isize,
    R16_UINT = RUSH_GFX_FORMAT_R16_UINT as isize,
    R32_FLOAT = RUSH_GFX_FORMAT_R32_FLOAT as isize,
    R32_UINT = RUSH_GFX_FORMAT_R32_UINT as isize,
    RG8_UNORM = RUSH_GFX_FORMAT_RG8_UNORM as isize,
    RG16_FLOAT = RUSH_GFX_FORMAT_RG16_FLOAT as isize,
    RG32_FLOAT = RUSH_GFX_FORMAT_RG32_FLOAT as isize,
    RGB32_FLOAT = RUSH_GFX_FORMAT_RGB32_FLOAT as isize,
    RGB8_UNORM = RUSH_GFX_FORMAT_RGB8_UNORM as isize,
    RGBA16_FLOAT = RUSH_GFX_FORMAT_RGBA16_FLOAT as isize,
    RGBA16_UNORM = RUSH_GFX_FORMAT_RGBA16_UNORM as isize,
    RGBA32_FLOAT = RUSH_GFX_FORMAT_RGBA32_FLOAT as isize,
    RGBA8_UNORM = RUSH_GFX_FORMAT_RGBA8_UNORM as isize,
    RGBA8_SRGB = RUSH_GFX_FORMAT_RGBA8_SRGB as isize,
    BGRA8_UNORM = RUSH_GFX_FORMAT_BGRA8_UNORM as isize,
    BGRA8_SRGB = RUSH_GFX_FORMAT_BGRA8_SRGB as isize,
    BC1_UNORM = RUSH_GFX_FORMAT_BC1_UNORM as isize,
    BC1_UNORM_SRGB = RUSH_GFX_FORMAT_BC1_UNORM_SRGB as isize,
    BC3_UNORM = RUSH_GFX_FORMAT_BC3_UNORM as isize,
    BC3_UNORM_SRGB = RUSH_GFX_FORMAT_BC3_UNORM_SRGB as isize,
    BC4_UNORM = RUSH_GFX_FORMAT_BC4_UNORM as isize,
    BC5_UNORM = RUSH_GFX_FORMAT_BC5_UNORM as isize,
    BC6H_UFLOAT = RUSH_GFX_FORMAT_BC6H_UFLOAT as isize,
    BC6H_SFLOAT = RUSH_GFX_FORMAT_BC6H_SFLOAT as isize,
    BC7_UNORM = RUSH_GFX_FORMAT_BC7_UNORM as isize,
    BC7_UNORM_SRGB = RUSH_GFX_FORMAT_BC7_UNORM_SRGB as isize,
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
        const NONE = RUSH_GFX_PASS_NONE;
        const CLEAR_COLOR = RUSH_GFX_PASS_CLEAR_COLOR;
        const CLEAR_DEPTH_STENCIL = RUSH_GFX_PASS_CLEAR_DEPTH_STENCIL;
        const DISCARD_COLOR = RUSH_GFX_PASS_DISCARD_COLOR;
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
        const NONE = RUSH_GFX_BUFFER_FLAG_NONE;
        const VERTEX = RUSH_GFX_BUFFER_FLAG_VERTEX;
        const INDEX = RUSH_GFX_BUFFER_FLAG_INDEX;
        const CONSTANT = RUSH_GFX_BUFFER_FLAG_CONSTANT;
        const STORAGE = RUSH_GFX_BUFFER_FLAG_STORAGE;
        const TEXEL = RUSH_GFX_BUFFER_FLAG_TEXEL;
        const INDIRECT_ARGS = RUSH_GFX_BUFFER_FLAG_INDIRECT_ARGS;
        const RAYTRACING = RUSH_GFX_BUFFER_FLAG_RAYTRACING;
        const TRANSIENT = RUSH_GFX_BUFFER_FLAG_TRANSIENT;
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

pub struct GfxDescriptorSetDesc {
    pub constant_buffers: u16,
    pub samplers: u16,
    pub textures: u16,
    pub rw_images: u16,
    pub rw_buffers: u16,
    pub rw_typed_buffers: u16,
    pub acceleration_structures: u16,
    pub stage_flags: rush_gfx_stage_flags,
    pub flags: rush_gfx_descriptor_set_flags,
}

pub struct GfxTechniqueDesc {
    //pub cs: rush_gfx_compute_shader,
    pub ps: GfxPixelShader,
    //pub gs: rush_gfx_geometry_shader,
    pub vs: GfxVertexShader,
    //pub ms: rush_gfx_mesh_shader,
    //pub vf: rush_gfx_vertex_format,
    //pub bindings: rush_gfx_shader_bindings_desc,
    pub work_group_size: [u16; 3usize],
    //pub spec_constant_count: u32,
    //pub spec_constants: *const rush_gfx_spec_constant,
    //pub spec_data: *const ::std::os::raw::c_void,
    //pub spec_data_size: u32,
}

/*
impl From<&GfxTechniqueDesc> for rush_gfx_technique_desc {
    fn from(desc: &GfxTechniqueDesc) -> Self {
        rush_gfx_technique_desc {
            cs: rush_gfx_compute_shader{handle: 0},
            ps: desc.ps.native,
            gs: rush_gfx_geometry_shader{handle: 0},
            vs: desc.vs.native,
            ms: rush_gfx_mesh_shader{handle: 0},
            vf: rush_gfx_vertex_format{handle: 0},
            //bindings: rush_gfx_shader_bindings_desc,
            work_group_size: desc.work_group_size,
            spec_constant_count: 0,
            spec_constants: std::ptr::null() as *const rush_gfx_spec_constant,
            spec_data: std::ptr::null() as *const ::std::os::raw::c_void,
            spec_data_size: 0, 
        }
    }
}
*/