use crate::gfx_context::*;

enum SamplerState {
    Point,
    Linear,
}

#[derive(Debug, PartialEq)]
enum BatchMode {
    Invalid,
    Batch2D,
    Batch3D,
}

struct BatchVertex {
    pos: (f32, f32, f32),
    tex: (f32, f32),
}

pub struct GfxPrimitiveBatch {
    batch_mode: BatchMode,
    max_batch_vertices: u32,
    vertices: Vec<BatchVertex>,
    ctx: Option<GfxContext>,
}

impl GfxPrimitiveBatch {
    pub fn new() -> GfxPrimitiveBatch {
        let default_batch_vertices: u32 = 12000;
        GfxPrimitiveBatch {
            batch_mode: BatchMode::Invalid,
            max_batch_vertices: default_batch_vertices,
            vertices: Vec::with_capacity(default_batch_vertices as usize),
            ctx: None,
        }
    }

    pub fn begin_2d(_width: f32, _height: f32) {}
    pub fn end_2d() {}

    fn flush(&mut self) {
        assert_ne!(self.batch_mode, BatchMode::Invalid);
        if self.vertices.is_empty() {
            return;
        }

        let ctx = self.ctx.as_mut().unwrap();

        /*
        GfxTechnique nextTechnique = getNextTechnique();

        Gfx_SetTechnique(m_context, nextTechnique);
        Gfx_UpdateBuffer(m_context, m_vertexBuffer, m_vertices.data(), (u32)m_vertices.sizeInBytes());
        Gfx_SetTexture(m_context, 0, m_currTexture);
        Gfx_SetSampler(m_context, 0, m_currSampler);
        Gfx_SetVertexStream(m_context, 0, m_vertexBuffer);
        Gfx_SetPrimitive(m_context, m_currPrim);

        if (m_constantBuffer.valid())
        {
            if (m_constantBufferDirty)
            {
                Gfx_UpdateBuffer(m_context, m_constantBuffer, &m_constants, sizeof(m_constants));
                m_constantBufferDirty = false;
            }
            Gfx_SetConstantBuffer(m_context, 0, m_constantBuffer);
        }
        */

        ctx.draw(0, self.vertices.len() as u32);

        self.vertices.clear();
    }
}
