use shapes::rectangle::Rectangle;

use crate::TextureId;

pub struct Sprite {
    pub texture_id: Option<TextureId>,
    pub rect: Option<Rectangle>,
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
