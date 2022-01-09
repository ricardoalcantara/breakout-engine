use image::{DynamicImage, Rgba};
use log::warn;
use std::{collections::HashMap, path::Path};

use crate::{
    error::{BreakoutError, BreakoutResult},
    shapes::rectangle::Rect,
};
extern crate freetype;

struct GlyphImage {
    size: glam::IVec2,
    bearing: glam::IVec2,
    advance: u32,
    character: char,
    buffer: Vec<u8>,
}

pub struct Character {
    position: glam::IVec2,
    size: glam::IVec2,
    bearing: glam::IVec2,
    advance: u32,
}

pub struct FontAtlas<T> {
    texture: T,
    characters: HashMap<char, Character>,
    line_spacing: u32,
}

pub struct Font<T> {
    face: freetype::Face,
    atlas: HashMap<u32, FontAtlas<T>>,
}

impl<T> Font<T> {
    pub fn new(path: &str) -> BreakoutResult<Font<T>> {
        let lib = freetype::Library::init().map_err(BreakoutError::FontError)?;
        let font_name = Path::new(path);
        if !font_name.exists() {
            panic!("ERROR::FREETYPE: Failed to load font_name");
        }
        let face = lib
            .new_face(font_name, 0)
            .map_err(BreakoutError::FontError)?;

        let atlas = HashMap::new();
        Ok(Font { face, atlas })
    }

    pub fn new_from_memory(buffer: &[u8]) -> BreakoutResult<Font<T>> {
        let lib = freetype::Library::init().map_err(BreakoutError::FontError)?;
        let face = lib
            .new_memory_face(buffer.to_vec(), 0)
            .map_err(BreakoutError::FontError)?;

        let atlas = HashMap::new();
        Ok(Font { face, atlas })
    }

    pub fn has_size(&self, size: u32) -> bool {
        self.atlas.contains_key(&size)
    }

    pub fn build_with_size<F>(&mut self, size: u32, get_texture: F) -> BreakoutResult
    where
        F: FnOnce(DynamicImage) -> BreakoutResult<T>,
    {
        if self.has_size(size) {
            return Ok(());
        }
        // set size to load glyphs as
        self.face
            .set_pixel_sizes(0, size)
            .map_err(BreakoutError::FontError)?;

        let mut glyph_images = Vec::new();
        let mut max_width = 0;
        let mut max_height = 0;

        let mut chars = Vec::new();
        chars.extend(32..128usize);
        chars.extend(160..255usize);

        for c in chars {
            self.face
                .load_char(c, freetype::face::LoadFlag::RENDER)
                .map_err(BreakoutError::FontError)?;

            let bitmap = self.face.glyph().bitmap();
            let bytes = bitmap.buffer();

            let (width, height) = (bitmap.width(), bitmap.rows());

            glyph_images.push(GlyphImage {
                character: c as u8 as char,
                buffer: bytes.to_vec(),
                size: glam::ivec2(width, height),
                bearing: glam::ivec2(
                    self.face.glyph().bitmap_left(),
                    self.face.glyph().bitmap_top(),
                ),
                advance: self.face.glyph().advance().x as u32,
            });

            max_width = width.max(max_width);
            max_height = height.max(max_height);
        }

        let spacing = 4;
        println!("Tile: {}, {}", max_width, max_height);
        let columns = 10; // 512 / (max_width + spacing);
        let rows = (glyph_images.len() as i32 / columns) + 1;
        let image_width = columns * max_width + spacing * columns;
        let image_height = rows * max_height + spacing * rows;
        println!("Image: {}, {}", image_width, image_height);

        let mut image = DynamicImage::new_rgba8(image_width as u32, image_height as u32).to_rgba8();
        let mut characters = HashMap::new();

        for (index, glyph_image) in glyph_images.iter().enumerate() {
            let x = index as i32 % columns;
            let y = index as i32 / columns;

            let x_offset = x * (max_width + spacing);
            let y_offset = y * (max_height + spacing);

            characters.insert(
                glyph_image.character,
                Character {
                    size: glyph_image.size,
                    advance: glyph_image.advance,
                    bearing: glyph_image.bearing,
                    position: glam::ivec2(x_offset, y_offset),
                },
            );
            let width = glyph_image.size.x;
            for (i, b) in glyph_image.buffer.iter().enumerate() {
                let x = i as u32 % width as u32;
                let y = i as u32 / width as u32;
                let px = image.get_pixel_mut(x_offset as u32 + x, y_offset as u32 + y);
                *px = Rgba([255, 255, 255, *b as u8]);
            }
        }

        // image.save("debug_font_atlas.png").unwrap();
        let texture = get_texture(DynamicImage::ImageRgba8(image))?;

        let line_spacing = if let Some(metrics) = self.face.size_metrics() {
            metrics.height as u32
        } else {
            warn!("It whould have the size_metrics");
            self.face.height() as u32
        };

        let font_atlas = FontAtlas::<T> {
            texture,
            characters,
            line_spacing,
        };

        self.atlas.insert(size, font_atlas);

        Ok(())
    }

    pub fn draw<F>(&self, text: &str, size: u32, mut render: F)
    where
        F: FnMut(&T, glam::Vec2, Rect),
    {
        if !self.atlas.contains_key(&size) {
            warn!("Font should be build before");
            return;
        }

        let atlas = &self.atlas[&size];
        let scale = 1.0;
        let mut x_pos = 0.0;
        let mut y_pos = 0.0;

        let h_bearing_y = &atlas.characters[&'H'].bearing.y;

        for c in text.chars() {
            if c.is_control() {
                if c == '\n' {
                    x_pos = 0.0;
                    y_pos += (atlas.line_spacing >> 6) as f32;
                    continue;
                }
            }

            if c.is_whitespace() {}

            let character = &atlas.characters[&c];

            // TODO: Should it bearing the first caractar?
            let xpos = x_pos + character.bearing.x as f32 * scale;
            let ypos = y_pos + (h_bearing_y - character.bearing.y) as f32 * scale;

            render(
                &atlas.texture,
                glam::vec2(xpos, ypos),
                Rect::from_position_size(
                    character.position.as_vec2().into(),
                    character.size.as_vec2().into(),
                ),
            );
            x_pos += (character.advance >> 6) as f32 * scale;
        }
    }
}
