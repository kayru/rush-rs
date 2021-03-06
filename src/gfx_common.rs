#![allow(non_camel_case_types)]

extern crate rush_sys;
use rush_sys::*;

macro_rules! declare_resource_type {
    ($name:ident, $handle:ident, $native:ident, $release:ident) => {
        pub struct $name {
            pub native: $native,
        }
        pub trait $handle {
            fn native(&self) -> $native;
        }
        impl $handle for $name {
            fn native(&self) -> $native {
                self.native
            }
        }
        impl $handle for $native {
            fn native(&self) -> $native {
                *self
            }
        }
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { $release(self.native) };
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name {
                    native: $native { handle: 0 },
                }
            }
        }
    };
}

#[rustfmt::skip] declare_resource_type!(GfxVertexFormat, GfxVertexFormatHandle, rush_gfx_vertex_format, rush_gfx_release_vertex_format);
#[rustfmt::skip] declare_resource_type!(GfxVertexShader, GfxVertexShaderHandle, rush_gfx_vertex_shader, rush_gfx_release_vertex_shader);
#[rustfmt::skip] declare_resource_type!(GfxPixelShader, GfxPixelShaderHandle, rush_gfx_pixel_shader, rush_gfx_release_pixel_shader);
#[rustfmt::skip] declare_resource_type!(GfxGeometryShader, GfxGeometryShaderHandle, rush_gfx_geometry_shader, rush_gfx_release_geometry_shader);
#[rustfmt::skip] declare_resource_type!(GfxComputeShader, GfxComputeShaderHandle, rush_gfx_compute_shader, rush_gfx_release_compute_shader);
#[rustfmt::skip] declare_resource_type!(GfxMeshShader, GfxMeshShaderHandle, rush_gfx_mesh_shader, rush_gfx_release_mesh_shader);
#[rustfmt::skip] declare_resource_type!(GfxRayTracingPipeline, GfxRayTracingPipelineHandle, rush_gfx_ray_tracing_pipeline, rush_gfx_release_ray_tracing_pipeline);
#[rustfmt::skip] declare_resource_type!(GfxAccelerationStructure, GfxAccelerationStructureHandle, rush_gfx_acceleration_structure, rush_gfx_release_acceleration_structure);
#[rustfmt::skip] declare_resource_type!(GfxTexture, GfxTextureHandle, rush_gfx_texture, rush_gfx_release_texture);
#[rustfmt::skip] declare_resource_type!(GfxBuffer, GfxBufferHandle, rush_gfx_buffer, rush_gfx_release_buffer);
#[rustfmt::skip] declare_resource_type!(GfxSampler, GfxSamplerHandle, rush_gfx_sampler, rush_gfx_release_sampler);
#[rustfmt::skip] declare_resource_type!(GfxBlendState, GfxBlendStateHandle, rush_gfx_blend_state, rush_gfx_release_blend_state);
#[rustfmt::skip] declare_resource_type!(GfxDepthStencilState, GfxDepthStencilStateHandle, rush_gfx_depth_stencil_state, rush_gfx_release_depth_stencil_state);
#[rustfmt::skip] declare_resource_type!(GfxRasterizerState, GfxRasterizerStateHandle, rush_gfx_rasterizer_state, rush_gfx_release_rasterizer_state);
#[rustfmt::skip] declare_resource_type!(GfxTechnique, GfxTechniqueHandle, rush_gfx_technique, rush_gfx_release_technique);
#[rustfmt::skip] declare_resource_type!(GfxDescriptorSet, GfxDescriptorSetHandle, rush_gfx_descriptor_set, rush_gfx_release_descriptor_set);

#[derive(Copy, Clone, PartialEq)]
pub enum GfxPrimitiveType {
    PointList = RUSH_GFX_PRIMITIVE_POINT_LIST as isize,
    LineList = RUSH_GFX_PRIMITIVE_LINE_LIST as isize,
    LineStrip = RUSH_GFX_PRIMITIVE_LINE_STRIP as isize,
    TriangleList = RUSH_GFX_PRIMITIVE_TRIANGLE_LIST as isize,
    TriangleStrip = RUSH_GFX_PRIMITIVE_TRIANGLE_STRIP as isize,
}

#[derive(Copy, Clone, PartialEq)]
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

impl GfxVertexFormat {
    pub fn new(elements: &[rush_gfx_vertex_element]) -> Self {
        GfxVertexFormat {
            native: unsafe { rush_gfx_create_vertex_format(elements.as_ptr(), elements.len() as u32) }
        } 
    }
}

pub enum GfxEmbeddedVertexShader {
    Primitive2D,
    Primitive3D,
}

pub enum GfxEmbeddedPixelShader {
    PrimitivePlain,
    PrimitiveTextured,
}

impl GfxVertexShader {
    pub fn new_with_source(source: &rush_gfx_shader_source) -> Self {
        GfxVertexShader {
            native: unsafe { rush_gfx_create_vertex_shader(source) },
        }
    }
    pub fn new_embedded(id: GfxEmbeddedVertexShader) -> Self {
        let source = unsafe {
            rush_gfx_get_embedded_shader(match id {
                GfxEmbeddedVertexShader::Primitive2D => RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_2D_VS,
                GfxEmbeddedVertexShader::Primitive3D => RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_3D_VS,
            })
        };
        GfxVertexShader::new_with_source(&source)
    }
}

impl GfxPixelShader {
    pub fn new_with_source(source: &rush_gfx_shader_source) -> Self {
        GfxPixelShader {
            native: unsafe { rush_gfx_create_pixel_shader(source) },
        }
    }
    pub fn new_embedded(id: GfxEmbeddedPixelShader) -> Self {
        let source = unsafe {
            rush_gfx_get_embedded_shader(match id {
                GfxEmbeddedPixelShader::PrimitivePlain => {
                    RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_PLAIN_PS
                }
                GfxEmbeddedPixelShader::PrimitiveTextured => {
                    RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_TEXTURED_PS
                }
            })
        };
        GfxPixelShader::new_with_source(&source)
    }
}

impl GfxComputeShader {
    pub fn new_with_source(source: &rush_gfx_shader_source) -> Self {
        GfxComputeShader {
            native: unsafe { rush_gfx_create_compute_shader(source) },
        }
    }
}

bitflags! {
    pub struct GfxUsageFlags: u32 {
        const NONE = 0 as u32;
        const SHADER_RESOURCE = RUSH_GFX_USAGE_SHADER_RESOURCE as u32;
        const RENDER_TARGET = RUSH_GFX_USAGE_RENDER_TARGET as u32;
        const DEPTH_STENCIL = RUSH_GFX_USAGE_DEPTH_STENCIL as u32;
        const STORAGE_IMAGE = RUSH_GFX_USAGE_STORAGE_IMAGE as u32;
        const TRANSFER_SRC = RUSH_GFX_USAGE_TRANSFER_SRC as u32;
        const TRANSFER_DST = RUSH_GFX_USAGE_TRANSFER_DST as u32;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum GfxTextureType {
    Tex1D = RUSH_GFX_TEXTURE_TYPE_1D as isize,
    Tex2D = RUSH_GFX_TEXTURE_TYPE_2D as isize,
    Tex3D = RUSH_GFX_TEXTURE_TYPE_3D as isize,
    TexCUBE = RUSH_GFX_TEXTURE_TYPE_CUBE as isize,
    Tex1D_Array = RUSH_GFX_TEXTURE_TYPE_1D_ARRAY as isize,
    Tex2D_Array = RUSH_GFX_TEXTURE_TYPE_2D_ARRAY as isize,
    TexCUBE_Array = RUSH_GFX_TEXTURE_TYPE_CUBE_ARRAY as isize,
}

pub struct GfxTextureDesc {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mips: u32,
    pub samples: u32,
    pub format: GfxFormat,
    pub texture_type: GfxTextureType,
    pub usage: GfxUsageFlags,
}

impl From<&GfxTextureDesc> for rush_gfx_texture_desc {
    fn from(desc: &GfxTextureDesc) -> Self {
        rush_gfx_texture_desc {
            width: desc.width,
            height: desc.height,
            depth: desc.depth,
            mips: desc.mips,
            samples: desc.samples,
            format: desc.format as rush_gfx_format,
            texture_type: desc.texture_type as rush_gfx_texture_type,
            usage: desc.usage.bits() as rush_gfx_usage_flags,
            debug_name: std::ptr::null(),
        }
    }
}

impl GfxTexture {
    pub fn new(desc: &GfxTextureDesc) -> Self {
        let native_desc = rush_gfx_texture_desc::from(desc);
        GfxTexture {
            native: unsafe {
                rush_gfx_create_texture(&native_desc, std::ptr::null(), 0, std::ptr::null())
            },
        }
    }
    pub fn new_with_pixels<T>(desc: &GfxTextureDesc, pixels: *const T) -> Self {
        let native_desc = rush_gfx_texture_desc::from(desc);
        let texture_data = [rush_gfx_texture_data {
            offset: 0,
            pixels: std::ptr::null(),
            mip: 0,
            slice: 0,
            width: desc.width,
            height: desc.height,
            depth: desc.depth,
        }];
        GfxTexture {
            native: unsafe {
                rush_gfx_create_texture(
                    &native_desc,
                    texture_data.as_ptr(),
                    1,
                    pixels as *const ::std::os::raw::c_void,
                )
            },
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

#[derive(Copy, Clone)]
pub struct ColorRGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRGBA8 {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
    pub fn red() -> Self {
        ColorRGBA8::new(0xFF, 0x00, 0x00, 0xFF)
    }
    pub fn green() -> Self {
        ColorRGBA8::new(0x00, 0xFF, 0x00, 0xFF)
    }
    pub fn blue() -> Self {
        ColorRGBA8::new(0x00, 0x00, 0xFF, 0xFF)
    }
    pub fn white() -> Self {
        ColorRGBA8::new(0xFF, 0xFF, 0xFF, 0xFF)
    }
    pub fn black() -> Self {
        ColorRGBA8::black_alpha(0x00)
    }
    pub fn black_alpha(a: u8) -> Self {
        ColorRGBA8 {
            r: 0x00,
            g: 0x00,
            b: 0x00,
            a: a,
        }
    }
}

impl Default for ColorRGBA8 {
    fn default() -> Self {
        ColorRGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorRGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        ColorRGBA {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
    pub fn red() -> Self {
        ColorRGBA::new(1.0, 0.0, 0.0, 1.0)
    }
    pub fn green() -> Self {
        ColorRGBA::new(0.0, 1.0, 0.0, 1.0)
    }
    pub fn blue() -> Self {
        ColorRGBA::new(0.0, 0.0, 1.0, 1.0)
    }
    pub fn black() -> Self {
        ColorRGBA::black_alpha(0.0)
    }
    pub fn black_alpha(a: f32) -> Self {
        ColorRGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: a,
        }
    }
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
        const NONE = RUSH_GFX_PASS_NONE as u32;
        const CLEAR_COLOR = RUSH_GFX_PASS_CLEAR_COLOR as u32;
        const CLEAR_DEPTH_STENCIL = RUSH_GFX_PASS_CLEAR_DEPTH_STENCIL as u32;
        const DISCARD_COLOR = RUSH_GFX_PASS_DISCARD_COLOR as u32;
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
        const NONE = RUSH_GFX_BUFFER_FLAG_NONE as u32;
        const VERTEX = RUSH_GFX_BUFFER_FLAG_VERTEX as u32;
        const INDEX = RUSH_GFX_BUFFER_FLAG_INDEX as u32;
        const CONSTANT = RUSH_GFX_BUFFER_FLAG_CONSTANT as u32;
        const STORAGE = RUSH_GFX_BUFFER_FLAG_STORAGE as u32;
        const TEXEL = RUSH_GFX_BUFFER_FLAG_TEXEL as u32;
        const INDIRECT_ARGS = RUSH_GFX_BUFFER_FLAG_INDIRECT_ARGS as u32;
        const RAYTRACING = RUSH_GFX_BUFFER_FLAG_RAYTRACING as u32;
        const TRANSIENT = RUSH_GFX_BUFFER_FLAG_TRANSIENT as u32;
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
            flags: desc.flags.bits() as rush_gfx_buffer_flags,
            format: desc.format as rush_gfx_format,
            stride: desc.stride,
            count: desc.count,
            host_visible: desc.host_visible,
        }
    }
}

pub struct GfxTechniqueDesc {
    pub cs: GfxComputeShader,
    pub ps: GfxPixelShader,
    //pub gs: rush_gfx_geometry_shader,
    pub vs: GfxVertexShader,
    //pub ms: rush_gfx_mesh_shader,
    //pub vf: rush_gfx_vertex_format,
    //pub bindings: rush_gfx_shader_bindings_desc,
    pub work_group_size: [u16; 3],
    //pub spec_constant_count: u32,
    //pub spec_constants: *const rush_gfx_spec_constant,
    //pub spec_data: *const ::std::os::raw::c_void,
    //pub spec_data_size: u32,
}

impl GfxTechnique {
    pub fn new(desc: &rush_gfx_technique_desc) -> Self {
        GfxTechnique {
            native: unsafe { rush_gfx_create_technique(desc) }
        }
    }
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

impl GfxSampler {
    pub fn new(desc: &rush_gfx_sampler_desc) -> Self {
        GfxSampler {
            native: unsafe { rush_gfx_create_sampler_state(desc) }
        }
    }
}
