use shapes::rectangle::Rect;

use crate::TextureId;

pub struct Sprite {
    pub texture_id: Option<TextureId>,
    pub rect: Option<Rect>,
    pub color: Option<glam::Vec3>,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture_id: None,
            rect: None,
            color: None,
        }
    }
}
