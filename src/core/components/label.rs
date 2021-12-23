use crate::{core::asset_manager::FontId, render::texture::Texture};

pub struct Label {
    pub(crate) text: String,
    pub(crate) font_id: Option<FontId>,
    pub(crate) size: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) color: Option<glam::Vec3>,
    pub(crate) texture: Option<Texture>,
}

impl Label {
    pub fn new(text: String, font_id: FontId, size: f32) -> Label {
        Label {
            text,
            font_id: Some(font_id),
            size,
            width: 0.0,
            height: 0.0,
            color: None,
            texture: None,
        }
    }
}
