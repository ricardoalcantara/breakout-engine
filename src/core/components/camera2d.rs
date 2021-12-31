use crate::shapes::rectangle::Rect;

pub enum ScaleMode {
    Keep,
    KeepWidth,
    KeepHeight,
}

impl Default for ScaleMode {
    fn default() -> Self {
        ScaleMode::Keep
    }
}

#[derive(Default)]
pub struct Camera2D {
    pub width: i32,
    pub height: i32,
    pub scale_mode: ScaleMode,
}

impl Camera2D {
    pub fn new(width: i32, height: i32) -> Camera2D {
        Camera2D {
            width,
            height,
            ..Default::default()
        }
    }
    pub fn keep_width(width: i32) -> Camera2D {
        Camera2D {
            width,
            height: 0,
            scale_mode: ScaleMode::KeepWidth,
        }
    }

    pub fn keep_height(height: i32) -> Camera2D {
        Camera2D {
            width: 0,
            height,
            scale_mode: ScaleMode::KeepWidth,
        }
    }

    pub(crate) fn get_view_matrix(
        &self,
        window_size: &glam::UVec2,
        position: &glam::Vec2,
    ) -> glam::Mat4 {
        let rect = self.get_view_rect(window_size, position);
        glam::Mat4::orthographic_rh_gl(rect.x, rect.width, rect.height, rect.y, -1.0, 1.0)
    }

    pub fn get_view_rect(&self, window_size: &glam::UVec2, position: &glam::Vec2) -> Rect {
        let (x, y) = (position.x as i32, position.y as i32);
        match self.scale_mode {
            ScaleMode::Keep => todo!(),
            ScaleMode::KeepWidth => Rect::new(
                x as f32,
                y as f32,
                (self.width + x) as f32,
                (window_size.y as f32 * (self.width as f32 / window_size.x as f32)) + y as f32,
            ),
            ScaleMode::KeepHeight => Rect::new(
                x as f32,
                y as f32,
                (window_size.x as f32 * (self.height as f32 / window_size.y as f32)) + y as f32,
                (self.height + y) as f32,
            ),
        }
    }
}
