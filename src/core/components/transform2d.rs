pub struct Transform2D {
    pub position: glam::Vec2,
    pub rotate: f32,
    pub scale: glam::Vec2,
    pub pixel_snap: bool,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            rotate: 0.0,
            scale: glam::Vec2::ONE,
            pixel_snap: false,
        }
    }
}
