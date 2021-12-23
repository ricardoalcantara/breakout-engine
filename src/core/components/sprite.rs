use crate::{core::asset_manager::TextureId, shapes::rectangle::Rect};

pub struct Sprite {
    pub texture_id: Option<TextureId>,
    pub rect: Option<Rect>,
    pub color: Option<glam::Vec4>,
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
