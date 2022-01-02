use crate::{core::asset_manager::TextureId, shapes::rectangle::Rect};

pub struct Sprite {
    pub texture_id: Option<TextureId>,
    pub rect: Option<Rect>,
    pub color: Option<glam::Vec4>,
    pub visible: bool,
    pub center_origin: bool,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            texture_id: None,
            color: None,
            rect: None,
            visible: true,
            center_origin: false,
        }
    }
}
