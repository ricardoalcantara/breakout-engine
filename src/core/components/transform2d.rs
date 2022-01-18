pub struct Transform2D {
    pub(crate) position: glam::Vec2,
    pub(crate) rotate: f32,
    pub(crate) scale: glam::Vec2,
    pub(crate) dirt: bool,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            rotate: 0.0,
            scale: glam::Vec2::ONE,
            dirt: true,
        }
    }
}

impl Transform2D {
    pub fn new() -> Transform2D {
        Transform2D::default()
    }

    pub fn from_position(position: glam::Vec2) -> Transform2D {
        Transform2D {
            position,
            ..Default::default()
        }
    }

    pub fn from_position_rotation(position: glam::Vec2, rotate: f32) -> Transform2D {
        Transform2D {
            position,
            rotate,
            ..Default::default()
        }
    }

    pub fn from_position_rotation_scale(
        position: glam::Vec2,
        rotate: f32,
        scale: glam::Vec2,
    ) -> Transform2D {
        Transform2D {
            position,
            rotate,
            scale,
            ..Default::default()
        }
    }

    pub fn from_position_rotation_scale_pixel_snap(
        position: glam::Vec2,
        rotate: f32,
        scale: glam::Vec2,
        pixel_snap: bool,
    ) -> Transform2D {
        Transform2D {
            position,
            rotate,
            scale,
            ..Default::default()
        }
    }

    pub fn position(&self) -> glam::Vec2 {
        self.position
    }
    pub fn set_position(&mut self, position: glam::Vec2) {
        self.dirt = true;
        self.position = position;
    }
    pub fn rotate(&self) -> f32 {
        self.rotate
    }
    pub fn set_rotate(&mut self, rotate: f32) {
        self.dirt = true;
        self.rotate = rotate;
    }
    pub fn scale(&self) -> glam::Vec2 {
        self.scale
    }
    pub fn set_scale(&mut self, scale: glam::Vec2) {
        self.dirt = true;
        self.scale = scale;
    }

    pub fn translate(&mut self, position: glam::Vec2) {
        self.dirt = true;
        self.position += position;
    }
}
