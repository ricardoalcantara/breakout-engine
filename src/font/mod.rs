use crate::shapes::rectangle::Rect;
use ab_glyph::{Font as ABFont, FontVec, Glyph, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};
use std::collections::HashMap;

struct GlyphImage {
    glyph: Glyph,
    size: (u32, u32),
    character: char,
    pixels: Vec<GlyphPixel>,
}

struct GlyphPixel {
    x: u32,
    y: u32,
    // from 0.0 to 1.0
    p: f32,
}

pub struct Character {
    glyph: Glyph,
    bounds: Rect,
    size: glam::Vec2,
}

pub struct FontAtlas<T> {
    texture: T,
    characters: HashMap<char, Character>,
}

pub struct Font<T> {
    font: FontVec,
    atlas: HashMap<String, FontAtlas<T>>,
}

impl<T> Font<T> {
    pub fn new(path: &str) -> Font<T> {
        let buf = std::fs::read(path).unwrap();
        let font = FontVec::try_from_vec(buf).unwrap();

        let atlas = HashMap::new();

        Font { font, atlas }
    }

    pub fn build_with_size<F>(&mut self, size: f32, get_texture: F)
    where
        F: FnOnce(DynamicImage) -> T,
    {
        let scale = PxScale::from(size);
        let scaled_font = self.font.as_scaled(scale);

        let mut glyph_images = Vec::new();
        let mut max_width = 0;
        let mut max_height = 0;

        let mut chars = Vec::new();
        chars.extend(32..128u8);
        chars.extend(160..255u8);

        for character in chars.iter().map(|c| *c as char) {
            let glyph: Glyph = scaled_font.scaled_glyph(character);
            if let Some(outlined) = scaled_font.outline_glyph(glyph.clone()) {
                let mut pixels_color = Vec::new();

                outlined.draw(|x, y, p| {
                    pixels_color.push(GlyphPixel { x, y, p });
                });

                let bounds = outlined.px_bounds();
                let (width, height) = (bounds.width() as u32, bounds.height() as u32);

                glyph_images.push(GlyphImage {
                    glyph,
                    size: (width, height),
                    character,
                    pixels: pixels_color,
                });

                max_width = width.max(max_width);
                max_height = height.max(max_height);
            }
        }

        let spacing = 4;
        println!("Tile: {}, {}", max_width, max_height);
        let columns = 512 / (max_width + spacing);
        let rows = (glyph_images.len() as u32 / columns) + 1;
        let image_width = columns * max_width + spacing * columns;
        let image_height = rows * max_height + spacing * rows;
        println!("Image: {}, {}", image_width, image_height);

        let mut image = DynamicImage::new_rgba8(image_width, image_height).to_rgba8();
        let mut characters = HashMap::new();

        for (index, glyph_image) in glyph_images.iter().enumerate() {
            let x = index as u32 % columns;
            let y = index as u32 / columns;

            let x_offset = x * (max_width + spacing);
            let y_offset = y * (max_height + spacing);

            let (width, height) = glyph_image.size;
            characters.insert(
                glyph_image.character,
                Character {
                    glyph: glyph_image.glyph.clone(),
                    bounds: Rect::new(
                        x_offset as f32,
                        y_offset as f32,
                        width as f32,
                        height as f32,
                    ),
                    size: glam::vec2(glyph_image.size.0 as f32, glyph_image.size.1 as f32),
                },
            );

            for pixels in &glyph_image.pixels {
                let px = image.get_pixel_mut(x_offset + pixels.x, y_offset + pixels.y);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    255,
                    255,
                    255,
                    px.0[3].saturating_add((pixels.p * 255.0) as u8),
                ]);
            }
        }

        let texture = get_texture(DynamicImage::ImageRgba8(image));

        let font_atlas = FontAtlas::<T> {
            texture,
            characters,
        };

        self.atlas.insert(size.to_string(), font_atlas);
    }

    pub fn draw<F>(&self, text: &str, position: glam::Vec2, size: f32, mut render: F)
    where
        F: FnMut(&T, glam::Vec2, Rect),
    {
        let font = self.font.as_scaled(100.0);
        let atlas = self.atlas.values().next().unwrap();

        let v_advance = font.height() + font.line_gap();
        let mut cursor_position = glam::vec2(0.0, 0.0);

        let mut last_glyph: Option<Glyph> = None;

        for c in text.chars() {
            if c.is_control() {
                if c == '\n' {
                    cursor_position = glam::vec2(0.0, v_advance);
                    continue;
                }
            }

            if c.is_whitespace() {
                let glyph_id = self.font.glyph_id(c);

                cursor_position.x += font.h_advance(glyph_id);

                if let Some(previous) = last_glyph.take() {
                    cursor_position.x += font.kern(previous.id, glyph_id);
                }

                last_glyph = Some(font.scaled_glyph(c));

                continue;
            }

            let character = &atlas.characters[&c];
            let glyph = &character.glyph;

            if let Some(previous) = last_glyph.take() {
                cursor_position.x += font.kern(previous.id, glyph.id);
            }

            last_glyph = Some(glyph.clone());

            let mut glyph_position = cursor_position.clone();
            cursor_position.x += font.h_advance(glyph.id);

            glyph_position.y += v_advance - character.size.y;

            render(&atlas.texture, glyph_position, character.bounds);
        }
    }
}
