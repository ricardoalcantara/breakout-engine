use crate::shapes::rectangle::Rect;

pub enum ScaleMode {
    Keep,
    KeepWidth,
    KeepHeight,
    Expand,
}

impl Default for ScaleMode {
    fn default() -> Self {
        ScaleMode::Keep
    }
}

#[derive(PartialEq, Eq)]
pub enum AnchorMode {
    TopLeft,
    Center,
}
impl Default for AnchorMode {
    fn default() -> Self {
        AnchorMode::TopLeft
    }
}

#[derive(Default)]
pub struct Camera2D {
    pub scale_x: f32,
    pub scale_y: f32,
    pub offset: glam::IVec2,
    pub scale_mode: ScaleMode,
    pub anchor_mode: AnchorMode,
}

impl Camera2D {
    pub fn new(scale_x: f32, scale_y: f32) -> Camera2D {
        Camera2D {
            scale_x,
            scale_y,
            ..Default::default()
        }
    }
    pub fn keep_width(scale_x: f32) -> Camera2D {
        Camera2D {
            scale_x,
            scale_y: 0.0,
            scale_mode: ScaleMode::KeepWidth,
            ..Default::default()
        }
    }

    pub fn keep_height(scale_y: f32) -> Camera2D {
        Camera2D {
            scale_x: 0.0,
            scale_y,
            scale_mode: ScaleMode::KeepHeight,
            ..Default::default()
        }
    }

    pub fn new_expand(width: f32, height: f32) -> Camera2D {
        Camera2D {
            scale_x: width,
            scale_y: height,
            scale_mode: ScaleMode::Expand,
            ..Default::default()
        }
    }

    pub(crate) fn get_view_matrix(
        &self,
        window_size: &glam::UVec2,
        position: &glam::Vec2,
    ) -> glam::Mat4 {
        let rect = self.get_view_rect(window_size, position);
        glam::Mat4::orthographic_rh_gl(
            rect.x,
            rect.width + rect.x,
            rect.height + rect.y,
            rect.y,
            -1.0,
            1.0,
        )
    }

    pub fn get_view_rect(&self, window_size: &glam::UVec2, position: &glam::Vec2) -> Rect {
        let (x, y) = (
            position.x as i32 + self.offset.x,
            position.y as i32 + self.offset.y,
        );
        let mut rect = match self.scale_mode {
            ScaleMode::Keep => {
                Rect::new(x as f32, y as f32, self.scale_x as f32, self.scale_y as f32)
            }
            ScaleMode::KeepWidth => {
                let width = self.scale_x * window_size.x as f32;
                Rect::new(
                    x as f32,
                    y as f32,
                    width,
                    window_size.y as f32 * (width as f32 / window_size.x as f32),
                )
            }
            ScaleMode::KeepHeight => {
                let height = self.scale_y * window_size.y as f32;
                Rect::new(
                    x as f32,
                    y as f32,
                    window_size.x as f32 * (height as f32 / window_size.y as f32),
                    height as f32,
                )
            }
            ScaleMode::Expand => Rect::new(
                x as f32,
                y as f32,
                self.scale_x * window_size.x as f32,
                self.scale_y * window_size.y as f32,
            ),
        };

        if AnchorMode::Center == self.anchor_mode {
            let half_size_x = rect.width / 2.0;
            let half_size_y = rect.height / 2.0;
            rect.x -= half_size_x;
            rect.y -= half_size_y;
        }

        rect
    }
}
