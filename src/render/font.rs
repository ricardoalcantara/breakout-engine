use crate::error::{BreakoutError, BreakoutResult};
use ab_glyph::{Font as ABFont, FontVec, Glyph, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};

pub struct Font {
    font: FontVec,
}

impl Font {
    pub fn new(path: &str) -> BreakoutResult<Font> {
        let buf = std::fs::read(path).map_err(BreakoutError::IOError)?;
        let font = FontVec::try_from_vec(buf).map_err(BreakoutError::InvalidFont)?;
        Ok(Font { font })
    }

    pub fn new_from_bytes(buf: &[u8]) -> BreakoutResult<Font> {
        let font = FontVec::try_from_vec(buf.into()).map_err(BreakoutError::InvalidFont)?;
        Ok(Font { font })
    }

    pub(crate) fn get_texture_from_text(&self, text: &str, size: f32) -> DynamicImage {
        // The font size to use
        let scale = PxScale::from(size);

        let scaled_font = self.font.as_scaled(scale);

        let mut glyphs = Vec::new();
        layout_paragraph(scaled_font, glam::vec2(0.0, 0.0), 9999.0, text, &mut glyphs);

        // Use a dark red colour
        let colour = (255, 255, 255);

        // work out the layout size
        let glyphs_height = scaled_font.height().ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs.first().unwrap().position.x;
            let last_glyph = glyphs.last().unwrap();
            let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
            (max_x - min_x).ceil() as u32
        };

        // Create a new rgba image with some padding
        let mut image = DynamicImage::new_rgba8(glyphs_width, glyphs_height).to_rgba8();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                // Draw the glyph into the image per-pixel by using the draw closure
                outlined.draw(|x, y, v| {
                    // Offset the position by the glyph bounding box
                    let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                    // Turn the coverage into an alpha value (blended with any previous)
                    *px = Rgba([
                        colour.0,
                        colour.1,
                        colour.2,
                        px.0[3].saturating_add((v * 255.0) as u8),
                    ]);
                });
            }
        }

        DynamicImage::ImageRgba8(image)
    }
}

fn layout_paragraph<F, SF>(
    font: SF,
    position: glam::Vec2,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: ABFont,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + glam::vec2(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = glam::vec2(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = ab_glyph::point(caret.x, caret.y);

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = glam::vec2(position.x, caret.y + v_advance);
            glyph.position = ab_glyph::point(caret.x, caret.y);
            last_glyph = None;
        }

        target.push(glyph);
    }
}
