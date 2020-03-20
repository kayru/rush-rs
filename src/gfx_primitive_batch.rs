#![allow(dead_code, unused_macros, unused_variables)]

use crate::gfx_common::*;
use crate::gfx_context::*;
use crate::rush_sys::*;

pub enum GfxBatchSampler {
    Point,
    Linear,
}

#[derive(Debug, PartialEq)]
enum BatchMode {
    Invalid,
    Batch2D,
    Batch3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GfxBatchVertex {
    pos: (f32, f32, f32),
    tex: (f32, f32),
    col: ColorRGBA8,
}

#[derive(Copy, Clone, PartialEq)]
enum TechniqueID {
    Plain2D = 0,
    Plain3D = 1,
    Textured2D = 2,
    Textured3D = 3,
    COUNT = 4,
}

const INVALID_TECHNIQUE_ID: TechniqueID = TechniqueID::COUNT;

fn is_textured(id: &TechniqueID) -> bool {
    match id {
        TechniqueID::Textured2D => true,
        TechniqueID::Textured3D => true,
        _ => false,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Constants {
    transform3d: [f32; 16], // mat4
    transform2d: [f32; 4],  // xy: scale, zw: bias
    color: [f32; 4],
}

impl Default for Constants {
    fn default() -> Self {
        Constants {
            transform3d: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            transform2d: [1.0, 1.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub struct GfxPrimitiveBatch {
    batch_mode: BatchMode,
    current_technique: TechniqueID,
    current_primitive: GfxPrimitiveType,
    current_texture: rush_gfx_texture,
    max_batch_vertices: u32,
    batch_vertex_count: u32,
    vertices: Vec<GfxBatchVertex>,
    techniques: [GfxTechnique; TechniqueID::COUNT as usize],
    constants: Constants,
    constants_dirty: bool,
    constant_buffer: GfxBuffer,
    vertex_buffer: GfxBuffer,
    sampler_linear: GfxSampler,
    sampler_point: GfxSampler,
}

impl GfxPrimitiveBatch {
    pub fn begin_2d_scale_bias(&mut self, scale: (f32, f32), bias: (f32, f32)) {
        self.batch_mode = BatchMode::Batch2D;
        self.constants.transform2d = [scale.0, scale.1, bias.0, bias.1];
        self.constants_dirty = true;
    }
    pub fn begin_2d(&mut self, (width, height): (f32, f32)) {
        let scale = (2.0 / width, -2.0 / height);
        let bias = (-1.0, 1.0);
        self.begin_2d_scale_bias(scale, bias);
    }
    pub fn end_2d(&mut self, ctx: &mut GfxContext) {
        self.flush(ctx);
        self.batch_mode = BatchMode::Invalid;
        self.current_technique = INVALID_TECHNIQUE_ID;
        self.current_texture = rush_gfx_texture { handle: 0 };
    }

    fn set_technique(&mut self, ctx: &mut GfxContext, technique: TechniqueID) {
        if self.current_technique != technique {
            self.flush(ctx);
            self.current_technique = technique;
        }
    }

    pub fn draw_line_2d(
        &mut self,
        ctx: &mut GfxContext,
        (x0, y0, x1, y1): (f32, f32, f32, f32),
        (color0, color1): (ColorRGBA8, ColorRGBA8),
    ) {
        self.set_technique(ctx, TechniqueID::Plain2D);
        let mut vertices = self.draw_vertices(ctx, GfxPrimitiveType::LineList, 2);

        vertices[0].pos = (x0, y0, 0.0);
        vertices[0].tex = (0.0, 0.0);
        vertices[0].col = color0;
        vertices[1].pos = (x1, y1, 0.0);
        vertices[1].tex = (0.0, 0.0);
        vertices[1].col = color1;
    }

    pub fn draw_rect_2d(
        &mut self,
        ctx: &mut GfxContext,
        (tlx, tly, brx, bry): (f32, f32, f32, f32),
        color: ColorRGBA8,
    ) {
        self.set_technique(ctx, TechniqueID::Plain2D);

        let mut vertices = self.draw_vertices(ctx, GfxPrimitiveType::TriangleList, 6);

        vertices[0].pos = (tlx, tly, 0.0);
        vertices[0].tex = (0.0, 0.0);
        vertices[0].col = color;

        vertices[1].pos = (tlx, bry, 0.0);
        vertices[1].tex = (0.0, 0.0);
        vertices[1].col = color;

        vertices[2].pos = (brx, bry, 0.0);
        vertices[2].tex = (0.0, 0.0);
        vertices[2].col = color;

        vertices[3] = vertices[0];
        vertices[4] = vertices[2];

        vertices[5].pos = (brx, tly, 0.0);
        vertices[5].tex = (0.0, 0.0);
        vertices[5].col = color;
    }

    pub fn draw_textured_rect_2d(
        &mut self,
        ctx: &mut GfxContext,
        (pos_tlx, pos_tly, pos_brx, pos_bry): (f32, f32, f32, f32),
        (tex_tlx, tex_tly, tex_brx, tex_bry): (f32, f32, f32, f32),
        color: ColorRGBA8,
    ) {
        self.set_technique(ctx, TechniqueID::Textured2D);

        let mut vertices = self.draw_vertices(ctx, GfxPrimitiveType::TriangleList, 6);

        vertices[0].pos = (pos_tlx, pos_tly, 0.0);
        vertices[0].tex = (tex_tlx, tex_tly);
        vertices[0].col = color;

        vertices[1].pos = (pos_tlx, pos_bry, 0.0);
        vertices[1].tex = (tex_tlx, tex_bry);
        vertices[1].col = color;

        vertices[2].pos = (pos_brx, pos_bry, 0.0);
        vertices[2].tex = (tex_brx, tex_bry);
        vertices[2].col = color;

        vertices[3] = vertices[0];
        vertices[4] = vertices[2];

        vertices[5].pos = (pos_brx, pos_tly, 0.0);
        vertices[5].tex = (tex_brx, tex_tly);
        vertices[5].col = color;
    }

    pub fn draw_tri_2d(
        &mut self,
        ctx: &mut GfxContext,
        (ax, ay): (f32, f32),
        (bx, by): (f32, f32),
        (cx, cy): (f32, f32),
        (colora, colorb, colorc): (ColorRGBA8, ColorRGBA8, ColorRGBA8),
    ) {
        self.set_technique(ctx, TechniqueID::Plain2D);

        let mut vertices = self.draw_vertices(ctx, GfxPrimitiveType::TriangleList, 3);

        vertices[0].pos = (ax, ay, 0.0);
        vertices[0].tex = (0.0, 0.0);
        vertices[0].col = colora;

        vertices[1].pos = (bx, by, 0.0);
        vertices[1].tex = (0.0, 0.0);
        vertices[1].col = colorb;

        vertices[2].pos = (cx, cy, 0.0);
        vertices[2].tex = (0.0, 0.0);
        vertices[2].col = colorc;
    }

    pub fn draw_vertices(
        &mut self,
        ctx: &mut GfxContext,
        primitive_type: GfxPrimitiveType,
        vertex_count: u32,
    ) -> &mut [GfxBatchVertex] {
        if vertex_count > self.max_batch_vertices {
            panic!(
                "vertex_count must be less than max_batch_vertices ({})",
                self.max_batch_vertices
            );
        }

        if primitive_type != self.current_primitive {
            self.flush(ctx);
            self.current_primitive = primitive_type;
        }

        if self.batch_vertex_count + vertex_count >= self.max_batch_vertices {
            self.flush(ctx);
        }

        let begin = self.batch_vertex_count as usize;
        let end = begin + vertex_count as usize;

        self.batch_vertex_count += vertex_count;
        &mut self.vertices[begin..end]
    }

    pub fn set_texture(
        &mut self,
        ctx: &mut GfxContext,
        texture: rush_gfx_texture,
        sampler: GfxBatchSampler,
    ) {
        if self.current_texture != texture {
            self.flush(ctx);
            self.current_texture = texture;
            ctx.set_texture(0, &texture);
            match sampler {
                GfxBatchSampler::Linear => ctx.set_sampler(0, &self.sampler_linear),
                GfxBatchSampler::Point => ctx.set_sampler(0, &self.sampler_point),
            };
        }
    }

    pub fn flush(&mut self, ctx: &mut GfxContext) {
        assert_ne!(self.batch_mode, BatchMode::Invalid);
        if self.batch_vertex_count == 0 {
            return;
        }
        if self.constants_dirty {
            let constants = [self.constants]; // todo: implement update_buffer that takes an object
            ctx.update_buffer_from_array(&self.constant_buffer, constants.as_ptr(), 1);
            self.constants_dirty = false;
        }
        ctx.set_constant_buffer(0, &self.constant_buffer, 0);

        ctx.update_buffer_from_array(
            &self.vertex_buffer,
            self.vertices.as_ptr(),
            self.batch_vertex_count,
        );
        ctx.set_vertex_buffer(0, &self.vertex_buffer);

        ctx.set_technique(&self.techniques[self.current_technique as usize]);
        ctx.set_primitive_type(self.current_primitive);
        ctx.draw(0, self.batch_vertex_count);

        self.batch_vertex_count = 0;
    }

    pub fn new() -> GfxPrimitiveBatch {
        let default_batch_vertices: u32 = 12000;

        let mut initial_vertices = Vec::new();
        initial_vertices.resize(
            default_batch_vertices as usize,
            GfxBatchVertex {
                pos: (0.0, 0.0, 0.0),
                tex: (0.0, 0.0),
                col: ColorRGBA8::black(),
            },
        );

        let vf_desc = [
            rush_gfx_vertex_element {
                semantic: RUSH_GFX_VERTEX_SEMANTIC_POSITION,
                index: 0,
                format: RUSH_GFX_FORMAT_RGB32_FLOAT,
                stream: 0,
            },
            rush_gfx_vertex_element {
                semantic: RUSH_GFX_VERTEX_SEMANTIC_TEXCOORD,
                index: 0,
                format: RUSH_GFX_FORMAT_RG32_FLOAT,
                stream: 0,
            },
            rush_gfx_vertex_element {
                semantic: RUSH_GFX_VERTEX_SEMANTIC_COLOR,
                index: 0,
                format: RUSH_GFX_FORMAT_RGBA8_UNORM,
                stream: 0,
            },
        ];

        let vf = unsafe { rush_gfx_create_vertex_format(vf_desc.as_ptr(), vf_desc.len() as u32) };

        let vs_2d = GfxVertexShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_2D_VS)
        });

        let vs_3d = GfxVertexShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_3D_VS)
        });

        let ps_plain = GfxPixelShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_PLAIN_PS)
        });

        let ps_textured = GfxPixelShader::new_with_source(unsafe {
            &rush_gfx_get_embedded_shader(RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_TEXTURED_PS)
        });

        let descriptor_sets_plain = [rush_gfx_descriptor_set_desc {
            constant_buffers: 1,
            ..Default::default()
        }];

        let descriptor_sets_textured = [rush_gfx_descriptor_set_desc {
            constant_buffers: 1,
            samplers: 1,
            textures: 1,
            ..Default::default()
        }];

        let technique_plain_2d = GfxTechnique {
            native: unsafe {
                rush_gfx_create_technique(&rush_gfx_technique_desc {
                    vs: vs_2d.native,
                    ps: ps_plain.native,
                    vf: vf,
                    bindings: rush_gfx_shader_bindings_desc {
                        descriptor_sets: descriptor_sets_plain.as_ptr(),
                        descriptor_set_count: descriptor_sets_plain.len() as u32,
                        use_default_descriptor_set: true,
                    },
                    ..Default::default()
                })
            },
        };

        let technique_plain_3d = GfxTechnique {
            native: unsafe {
                rush_gfx_create_technique(&rush_gfx_technique_desc {
                    vs: vs_3d.native,
                    ps: ps_plain.native,
                    vf: vf,
                    bindings: rush_gfx_shader_bindings_desc {
                        descriptor_sets: descriptor_sets_plain.as_ptr(),
                        descriptor_set_count: descriptor_sets_plain.len() as u32,
                        use_default_descriptor_set: true,
                    },
                    ..Default::default()
                })
            },
        };

        let technique_textured_2d = GfxTechnique {
            native: unsafe {
                rush_gfx_create_technique(&rush_gfx_technique_desc {
                    vs: vs_2d.native,
                    ps: ps_textured.native,
                    vf: vf,
                    bindings: rush_gfx_shader_bindings_desc {
                        descriptor_sets: descriptor_sets_textured.as_ptr(),
                        descriptor_set_count: descriptor_sets_textured.len() as u32,
                        use_default_descriptor_set: true,
                    },
                    ..Default::default()
                })
            },
        };

        let technique_textured_3d = GfxTechnique {
            native: unsafe {
                rush_gfx_create_technique(&rush_gfx_technique_desc {
                    vs: vs_3d.native,
                    ps: ps_textured.native,
                    vf: vf,
                    bindings: rush_gfx_shader_bindings_desc {
                        descriptor_sets: descriptor_sets_textured.as_ptr(),
                        descriptor_set_count: descriptor_sets_textured.len() as u32,
                        use_default_descriptor_set: true,
                    },
                    ..Default::default()
                })
            },
        };

        let constant_buffer = GfxBuffer::new(&GfxBufferDesc {
            flags: GfxBufferFlags::CONSTANT | GfxBufferFlags::TRANSIENT,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<Constants>() as u32,
            count: 1,
            host_visible: false,
        });

        let vertex_buffer = GfxBuffer::new(&GfxBufferDesc {
            flags: GfxBufferFlags::VERTEX | GfxBufferFlags::TRANSIENT,
            format: GfxFormat::UNKNOWN,
            stride: std::mem::size_of::<GfxBatchVertex>() as u32,
            count: default_batch_vertices,
            host_visible: false,
        });

        let sampler_linear_desc = rush_gfx_sampler_desc {
            filter_min: RUSH_GFX_TEXTURE_FILTER_LINEAR,
            filter_mag: RUSH_GFX_TEXTURE_FILTER_LINEAR,
            filter_mip: RUSH_GFX_TEXTURE_FILTER_LINEAR,
            wrap_u: RUSH_GFX_TEXTURE_WRAP_REPEAT,
            wrap_v: RUSH_GFX_TEXTURE_WRAP_REPEAT,
            wrap_w: RUSH_GFX_TEXTURE_WRAP_REPEAT,
            compare_func: RUSH_GFX_COMPARE_FUNC_ALWAYS,
            compare_enable: false,
            anisotropy: 1.0,
            mip_lod_bias: 0.0,
        };

        let sampler_point_desc = rush_gfx_sampler_desc {
            filter_min: RUSH_GFX_TEXTURE_FILTER_POINT,
            filter_mag: RUSH_GFX_TEXTURE_FILTER_POINT,
            filter_mip: RUSH_GFX_TEXTURE_FILTER_POINT,
            ..sampler_linear_desc
        };

        let sampler_linear = GfxSampler {
            native: unsafe { rush_gfx_create_sampler_state(&sampler_linear_desc) },
        };

        let sampler_point = GfxSampler {
            native: unsafe { rush_gfx_create_sampler_state(&sampler_point_desc) },
        };

        GfxPrimitiveBatch {
            batch_mode: BatchMode::Invalid,
            current_technique: INVALID_TECHNIQUE_ID,
            current_primitive: GfxPrimitiveType::LineList,
            current_texture: rush_gfx_texture { handle: 0 },
            max_batch_vertices: default_batch_vertices,
            batch_vertex_count: 0,
            vertices: initial_vertices,
            techniques: [
                technique_plain_2d,
                technique_plain_3d,
                technique_textured_2d,
                technique_textured_3d,
            ],
            constants: Constants::default(),
            constants_dirty: true,
            constant_buffer: constant_buffer,
            vertex_buffer: vertex_buffer,
            sampler_linear: sampler_linear,
            sampler_point: sampler_point,
        }
    }
}
