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
                    None => GfxTexture::default().native,
                },
            };
            color_targets.push(native);
        }
        let depth_target = rush_gfx_depth_target {
            target: match &desc.depth.target {
                Some(texture) => texture.native,
                None => GfxTexture::default().native,
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
                desc.flags.bits() as rush_gfx_buffer_flags,
            )
        };
    }

    pub fn end_pass(&mut self) {
        unsafe { rush_gfx_end_pass(self.native) };
    }

    pub fn set_technique<H: GfxTechniqueHandle>(&mut self, technique: &H) {
        unsafe { rush_gfx_set_technique(self.native, technique.native()) };
    }

    pub fn set_index_buffer<H: GfxBufferHandle>(&mut self, buffer: &H) {
        unsafe { rush_gfx_set_index_stream(self.native, buffer.native()) };
    }

    pub fn set_vertex_buffer<H: GfxBufferHandle>(&mut self, index: u32, buffer: &H) {
        unsafe { rush_gfx_set_vertex_stream(self.native, index, buffer.native()) };
    }

    pub fn set_constant_buffer<H: GfxBufferHandle>(&mut self, index: u32, buffer: &H, offset: u32) {
        unsafe { rush_gfx_set_constant_buffer(self.native, index, buffer.native(), offset) };
    }

    pub fn set_texture<H: GfxTextureHandle>(&mut self, slot: u32, texture: &H) {
        unsafe { rush_gfx_set_texture(self.native, slot, texture.native()) };
    }

    pub fn set_sampler<H: GfxSamplerHandle>(&mut self, slot: u32, sampler: &H) {
        unsafe { rush_gfx_set_sampler(self.native, slot, sampler.native()) };
    }

    pub fn set_storate_image<H: GfxTextureHandle>(&mut self, index: u32, image: &H) {
        unsafe { rush_gfx_set_storage_image(self.native, index, image.native()) };
    }

    pub fn set_primitive_type(&mut self, primitive_type: GfxPrimitiveType) {
        unsafe { rush_gfx_set_primitive(self.native, primitive_type as rush_gfx_primitive_type) };
    }

    pub fn update_buffer_from_array<T, H: GfxBufferHandle>(
        &mut self,
        buffer: &H,
        data: *const T,
        count: u32,
    ) {
        let elem_size = std::mem::size_of::<T>();
        let size_in_bytes = count as usize * elem_size;
        unsafe {
            rush_gfx_update_buffer(
                self.native,
                buffer.native(),
                data as *const ::std::os::raw::c_void,
                size_in_bytes as u32,
            )
        };
    }

    pub fn draw(&mut self, first_vertex: u32, vertex_count: u32) {
        unsafe { rush_gfx_draw(self.native, first_vertex, vertex_count) };
    }

    pub fn dispatch(&mut self, size_x: u32, size_y: u32, size_z: u32) {
        unsafe { rush_gfx_dispatch(self.native, size_x, size_y, size_z) };
    }
}
