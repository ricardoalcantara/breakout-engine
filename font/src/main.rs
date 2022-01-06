#![allow(unused)]
use std::collections::HashMap;

use ab_glyph::{point, Font as ABFont, FontVec, Glyph, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};

struct Character {
    index: u32,
    size: (u32, u32),
    bearing: (f32, f32),
    advance: f32,
}

pub struct FontBuilder {
    font: FontVec,
}

impl FontBuilder {
    pub fn new(path: &str) -> FontBuilder {
        let buf = std::fs::read(path).unwrap();
        let font = FontVec::try_from_vec(buf).unwrap();

        FontBuilder { font }
    }

    pub fn build_with_size(&self, size: f32) -> FontAtlas {
        let scale = PxScale::from(size);
        let scaled_font = self.font.as_scaled(scale);

        let mut characters: HashMap<char, Character> = HashMap::new();
        let mut glyph_images = Vec::new();
        let mut max_width = 0;
        let mut max_height = 0;
        // TODO: Temp
        let mut index = 0;
        for c in 32..128u8 {
            let glyph: Glyph = scaled_font.scaled_glyph(c as char);
            let glyph_id = glyph.id;
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let mut pixels_color = Vec::new();
                let mut width = 0;
                let mut height = 0;
                outlined.draw(|x, y, p| {
                    pixels_color.push((x, y, p));
                    width = x.max(width);
                    height = y.max(height);
                });
                glyph_images.push(pixels_color);

                max_width = width.max(max_width);
                max_height = height.max(max_height);

                let bounds = outlined.px_bounds();
                let character = Character {
                    index,
                    size: (width, height),
                    bearing: (0.0, 0.0),
                    advance: scaled_font.h_advance(glyph_id),
                };
                characters.insert(c as char, character);
                index += 1;
            }
        }
        for c in 160..255u8 {}

        let spacing = 4;
        println!("Tile: {}, {}", max_width, max_height);
        let columns = 512 / (max_width + spacing);
        let rows = (index / columns) + 1;
        let image_width = columns * max_width + spacing * columns;
        let image_height = rows * max_height + spacing * rows;
        println!("Image: {}, {}", image_width, image_height);

        let mut image = DynamicImage::new_rgba8(image_width, image_height).to_rgba8();

        for (i, pixels_color) in glyph_images.iter().enumerate() {
            let x = i as u32 % columns;
            let y = i as u32 / columns;
            println!("pc: {},{}", x, y);

            let x_offset = x * (max_width + spacing);
            let y_offset = y * (max_height + spacing);

            for pixels in pixels_color {
                let px = image.get_pixel_mut(x_offset + pixels.0, y_offset + pixels.1);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    255,
                    255,
                    255,
                    px.0[3].saturating_add((pixels.2 * 255.0) as u8),
                ]);
            }
        }

        // for c in 32..128u8 {
        //     let glyph: Glyph = scaled_font.scaled_glyph(c as char);
        //     if let Some(outlined) = scaled_font.outline_glyph(glyph) {
        //         let bounds = outlined.px_bounds();
        //         let width = (bounds.min.x + bounds.max.x) as u32;
        //         w += width + spacing;
        //         if w > 512 {
        //             w = 0;
        //             println!();
        //             h += glyphs_height + spacing;
        //         }
        //         print!(" {}", c as char);
        //     }
        // }

        // for c in 160..255u8 {
        //     let glyph: Glyph = scaled_font.scaled_glyph(c as char);
        //     if let Some(outlined) = scaled_font.outline_glyph(glyph) {
        //         let bounds = outlined.px_bounds();
        //         let width = (bounds.min.x + bounds.max.x) as u32;
        //         w += width + spacing;
        //         if w > 512 {
        //             w = 0;
        //             println!();
        //             h += glyphs_height + spacing;
        //         }
        //         print!(" {}", c as char);
        //     }
        // }

        // println!();
        // println!("Size: {} {}", 512, h);

        // let mut image = DynamicImage::new_rgba8(800, 1200).to_rgba8();

        // let mut w = 0;
        // let mut h = 0;

        // for c in 32..128u8 {
        //     let glyph: Glyph = scaled_font.scaled_glyph(c as char);
        //     if let Some(outlined) = scaled_font.outline_glyph(glyph) {
        //         let bounds = outlined.px_bounds();
        //         let width = (bounds.min.x + bounds.max.x) as u32;
        //         if (w + width) > 512 {
        //             w = 0;
        //             h += glyphs_height + spacing;
        //         }
        //         outlined.draw(|x, y, v| {
        //             // Offset the position by the glyph bounding box
        //             let px = image
        //                 .get_pixel_mut(x + w + bounds.min.x as u32, y + h + bounds.min.y as u32);
        //             // Turn the coverage into an alpha value (blended with any previous)
        //             *px = Rgba([255, 255, 255, px.0[3].saturating_add((v * 255.0) as u8)]);
        //         });
        //         w += width + spacing;
        //     }
        // }

        // for c in 160..255u8 {
        //     let glyph: Glyph = scaled_font.scaled_glyph(c as char);
        //     if let Some(outlined) = scaled_font.outline_glyph(glyph) {
        //         let bounds = outlined.px_bounds();
        //         let width = (bounds.min.x + bounds.max.x) as u32;
        //         if (w + width) > 512 {
        //             w = 0;
        //             h += glyphs_height + spacing;
        //         }
        //         outlined.draw(|x, y, v| {
        //             // Offset the position by the glyph bounding box
        //             let px = image
        //                 .get_pixel_mut(x + w + bounds.min.x as u32, y + h + bounds.min.y as u32);
        //             // Turn the coverage into an alpha value (blended with any previous)
        //             *px = Rgba([255, 255, 255, px.0[3].saturating_add((v * 255.0) as u8)]);
        //         });
        //         w += width + spacing;
        //     }
        // }

        image.save("image_example.png").unwrap();

        FontAtlas {}
    }
}

pub struct FontAtlas {}

impl FontAtlas {
    pub fn render(&self, text: &str) {}
}

fn main() {
    let font_path = "../assets/Roboto-Regular.ttf";

    let font_builder = FontBuilder::new(font_path);
    let font = font_builder.build_with_size(50.0);
}
