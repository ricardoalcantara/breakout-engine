use std::cell::RefMut;

use crate::render::{renderer::Renderer, RenderQuad};

use super::Constraints;

pub(crate) struct Panel {
    x: Constraints,
    y: Constraints,
    width: Constraints,
    height: Constraints,
}

impl Panel {
    pub(crate) fn new(
        x: Constraints,
        y: Constraints,
        width: Constraints,
        height: Constraints,
    ) -> Panel {
        Panel {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn position(&self) -> glam::Vec2 {
        let x = match self.x {
            Constraints::Pixel(x) => x,
            Constraints::Auto => 0,
            _ => 0,
        };
        let y = match self.y {
            Constraints::Pixel(y) => y,
            Constraints::Auto => 0,
            _ => 0,
        };

        glam::vec2(x as f32, y as f32)
    }

    pub(crate) fn size(&self, spacing: i32, elements_count: i32) -> glam::Vec2 {
        let width = match self.width {
            Constraints::Pixel(width) => width,
            Constraints::Auto => 400,
            _ => 0,
        };
        let height = match self.height {
            Constraints::Pixel(height) => height,
            Constraints::Auto => (spacing + 10) * elements_count + 10,
            _ => 0,
        };

        glam::vec2(width as f32, height as f32)
    }

    pub(crate) fn draw(
        &self,
        renderer: &mut RefMut<Renderer>,
        spacing: i32,
        elements_count: i32,
        // font: &Font,
    ) {
        let position = self.position();
        let size = self.size(spacing, elements_count);
        renderer.draw_quad(RenderQuad {
            size,
            position,
            scale: glam::Vec2::ONE,
            rotate: 0.0,
            center_origin: false,
            color: glam::vec4(0.01, 0.01, 0.01, 0.8),
        });
    }
}
