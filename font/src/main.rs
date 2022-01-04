#![allow(unused)]
use ab_glyph::{point, Font as ABFont, FontVec, Glyph, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};

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

        let glyphs_height = scaled_font.height().ceil() as u32;

        let spacing = 4;

        let mut w = 0;
        let mut h = 0;

        for c in 32..128u8 {
            let glyph: Glyph = scaled_font.scaled_glyph(c as char);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = (bounds.min.x + bounds.max.x) as u32;
                w += width + spacing;
                if w > 512 {
                    w = 0;
                    println!();
                    h += glyphs_height + spacing;
                }
                print!(" {}", c as char);
            }
        }

        for c in 160..255u8 {
            let glyph: Glyph = scaled_font.scaled_glyph(c as char);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = (bounds.min.x + bounds.max.x) as u32;
                w += width + spacing;
                if w > 512 {
                    w = 0;
                    println!();
                    h += glyphs_height + spacing;
                }
                print!(" {}", c as char);
            }
        }

        println!();
        println!("Size: {} {}", 512, h);

        let mut image = DynamicImage::new_rgba8(800, 1200).to_rgba8();

        let mut w = 0;
        let mut h = 0;

        for c in 32..128u8 {
            let glyph: Glyph = scaled_font.scaled_glyph(c as char);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = (bounds.min.x + bounds.max.x) as u32;
                if (w + width) > 512 {
                    w = 0;
                    h += glyphs_height + spacing;
                }
                outlined.draw(|x, y, v| {
                    // Offset the position by the glyph bounding box
                    let px = image
                        .get_pixel_mut(x + w + bounds.min.x as u32, y + h + bounds.min.y as u32);
                    // Turn the coverage into an alpha value (blended with any previous)
                    *px = Rgba([255, 255, 255, px.0[3].saturating_add((v * 255.0) as u8)]);
                });
                w += width + spacing;
            }
        }

        for c in 160..255u8 {
            let glyph: Glyph = scaled_font.scaled_glyph(c as char);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = (bounds.min.x + bounds.max.x) as u32;
                if (w + width) > 512 {
                    w = 0;
                    h += glyphs_height + spacing;
                }
                outlined.draw(|x, y, v| {
                    // Offset the position by the glyph bounding box
                    let px = image
                        .get_pixel_mut(x + w + bounds.min.x as u32, y + h + bounds.min.y as u32);
                    // Turn the coverage into an alpha value (blended with any previous)
                    *px = Rgba([255, 255, 255, px.0[3].saturating_add((v * 255.0) as u8)]);
                });
                w += width + spacing;
            }
        }

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
