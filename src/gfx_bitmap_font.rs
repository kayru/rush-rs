#![allow(dead_code, unused_macros, unused_variables, unused_imports, unused_mut)]

use crate::gfx_common::*;
use crate::gfx_context::*;
use crate::gfx_primitive_batch::*;
use crate::rush_sys::*;

use std::io::BufReader;

const INVALID_PAGE_ID: u8 = 0xFF;

#[derive(Copy, Clone)]
struct CharData {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    offset_x: i16,
    offset_y: i16,
    advance_x: i16,
    page: u8,
    chan: u8,
}

impl CharData {
    pub fn new() -> Self {
        CharData {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            offset_x: 0,
            offset_y: 0,
            advance_x: 0,
            page: INVALID_PAGE_ID,
            chan: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct CharRect {
    pos: (f32, f32, f32, f32), // tl_x, tl_y, br_x, br_y
    tex: (f32, f32, f32, f32),
}

impl CharRect {
    pub fn new() -> Self {
        CharRect {
            pos: (0.0, 0.0, 0.0, 0.0),
            tex: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

fn add(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

pub struct GfxBitmapFontDesc {
    chars: Vec<CharData>,
    rects: Vec<CharRect>,
    pages: Vec<String>,
    size: u32,
}

fn next_pow2(mut n: u32) -> u32 {
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n += 1;
    n
}

impl GfxBitmapFont {
    pub fn new_embedded(shadow: bool, (pad_x, pad_y): (u32, u32)) -> Self {
        let char_width = 6 + pad_x;
        let char_height = 8 + pad_y;
        let char_count = b'~' as u32 - b' ' as u32 + 1;
        let glyph_border = if shadow { 1 } else { 0 };
        let glyph_width = char_width + glyph_border;
        let glyph_height = char_height + glyph_border;

        let img_width = next_pow2(glyph_width * char_count) as u32;
        let img_height = next_pow2(glyph_height) as u32;

        let mut rects: Vec<CharRect> = Vec::new();
        rects.resize(256, CharRect::new());

        let mut chars: Vec<CharData> = Vec::new();
        chars.resize(256, CharData::new());

        let mut pixels: Vec<u32> = Vec::new();
        pixels.resize((img_width * img_height) as usize, 0);

        for n in 0..char_count {
            let c = n as u8 + b' ';
            let s: [char; 2] = [c as char, 0 as char];

            chars[c as usize] = CharData {
                x: (glyph_width * n) as u16,
                y: 0,
                width: glyph_width as u16,
                height: glyph_height as u16,
                offset_x: 0,
                offset_y: 0,
                advance_x: char_width as i16,
                page: 0,
                chan: 0x0F,
            };

            let tx0 = (n as f32 + 0.0) * glyph_width as f32 / img_width as f32;
            let tx1 = (n as f32 + 1.0) * glyph_width as f32 / img_width as f32;

            rects[c as usize] = CharRect {
                pos: (0.0, 0.0, glyph_width as f32, glyph_height as f32),
                tex: (tx0, 0.0, tx1, 1.0),
            };

            unsafe {
                if shadow {
                    #[rustfmt::skip] rush_embedded_font_blit_6x8(pixels.as_mut_ptr(), glyph_width * n + 1, img_width, 0xFF000000, s.as_ptr() as *const i8);
                    #[rustfmt::skip] rush_embedded_font_blit_6x8(pixels.as_mut_ptr(), glyph_width * n + img_width + 1, img_width, 0xFF000000, s.as_ptr() as *const i8);
                    #[rustfmt::skip] rush_embedded_font_blit_6x8(pixels.as_mut_ptr(), glyph_width * n + img_width, img_width, 0xFF000000, s.as_ptr() as *const i8);
                }
                #[rustfmt::skip] rush_embedded_font_blit_6x8(pixels.as_mut_ptr(), glyph_width * n, img_width, 0xFFFFFFFF, s.as_ptr() as *const i8);
            }
        }

        let texture_desc = GfxTextureDesc {
            width: img_width,
            height: img_height,
            depth: 1,
            mips: 1,
            samples: 1,
            format: GfxFormat::RGBA8_UNORM,
            texture_type: GfxTextureType::Tex2D,
            usage: GfxUsageFlags::SHADER_RESOURCE,
        };
        GfxBitmapFont {
            desc: GfxBitmapFontDesc {
                chars: chars,
                rects: rects,
                pages: Vec::new(),
                size: char_height,
            },
            texture: GfxTexture::new_with_pixels(&texture_desc, pixels.as_ptr()),
        }
    }
}

pub struct GfxBitmapFont {
    desc: GfxBitmapFontDesc,
    texture: GfxTexture,
}

pub struct GfxBitmapFontRenderer<'a> {
    ctx: &'a mut GfxContext,
    prim: &'a mut GfxPrimitiveBatch,
    font: &'a GfxBitmapFont,
    pos: (f32, f32),
    col: ColorRGBA8,
}

impl<'a> GfxBitmapFontRenderer<'a> {
    pub fn new(
        ctx: &'a mut GfxContext,
        prim: &'a mut GfxPrimitiveBatch,
        font: &'a GfxBitmapFont,
    ) -> Self {
        prim.flush(ctx);
        prim.set_texture(ctx, font.texture.native, GfxBatchSampler::Linear);
        GfxBitmapFontRenderer {
            ctx: ctx,
            prim: prim,
            font: font,
            pos: (0.0, 0.0),
            col: ColorRGBA8::white(),
        }
    }
    pub fn set_position(mut self, pos: (f32, f32)) -> Self {
        self.pos = pos;
        self
    }
    pub fn set_color(mut self, col: ColorRGBA8) -> Self {
        self.col = col;
        self
    }
    pub fn print(mut self, text: &str) -> Self {
        let (mut x, mut y): (f32, f32) = self.pos;
        for ch in text.chars() {
            let c = ch as usize;
            if ch == '\n' {
                y += self.font.desc.size as f32;
                x = self.pos.0;
                continue;
            }

            let char_data = &self.font.desc.chars[c];
            if char_data.page == INVALID_PAGE_ID {
                continue;
            }

            let rect = self.font.desc.rects[c];

            self.prim.draw_textured_rect_2d(self.ctx, add(rect.pos, (x, y, x, y)), rect.tex, self.col);

            x += char_data.advance_x as f32;
        }
        self.pos = (x, y);
        self
    }
}
