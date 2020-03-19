#![allow(dead_code, unused_macros, unused_variables, unused_imports, unused_mut)]

use crate::gfx_common::*;
use crate::gfx_context::*;
use crate::gfx_primitive_batch::*;
use crate::rush_sys::*;

use std::io::BufReader;

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
            page: 0,
            chan: 0,
        }
    }
}

pub struct GfxBitmapFontDesc {
    chars: Vec<CharData>,
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
        let char_count = '~' as u32 - ' ' as u32 + 1;
        let glyph_border = if shadow { 1 } else { 0 };
        let glyph_width = char_width + glyph_border;
        let glyph_height = char_height + glyph_border;

        let img_width = next_pow2(glyph_width * char_count) as u32;
        let img_height = next_pow2(glyph_height) as u32;

        let mut chars: Vec<CharData> = Vec::new();
        chars.resize(256, CharData::new());

        let mut pixels: Vec<u32> = Vec::new();
        pixels.resize((img_width*img_height) as usize, 0);

        for n in 0..char_count {
            let c = n as u8 + ' ' as u8;
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

            unsafe {
                if shadow {
                    rush_embedded_font_blit_6x8(
                        pixels.as_mut_ptr(),
                        glyph_width * n + 1,
                        img_width,
                        0xFF000000,
                        s.as_ptr() as *const i8,
                    );
                    rush_embedded_font_blit_6x8(
                        pixels.as_mut_ptr(),
                        glyph_width * n + img_width + 1,
                        img_width,
                        0xFF000000,
                        s.as_ptr() as *const i8,
                    );
                    rush_embedded_font_blit_6x8(
                        pixels.as_mut_ptr(),
                        glyph_width * n + img_width,
                        img_width,
                        0xFF000000,
                        s.as_ptr() as *const i8,
                    );
                }
                rush_embedded_font_blit_6x8(
                    pixels.as_mut_ptr(),
                    glyph_width * n,
                    img_width,
                    0xFFFFFFFF,
                    s.as_ptr() as *const i8,
                );
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
    pub fn new(ctx: &'a mut GfxContext, prim: &'a mut GfxPrimitiveBatch, font: &'a GfxBitmapFont) -> Self {
        //prim.set_texture(ctx, font.texture);
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
        self
    }
}
