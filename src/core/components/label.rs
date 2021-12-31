use crate::{core::asset_manager::FontId, render::texture::Texture};

pub struct Label {
    pub text: String,
    pub font_id: Option<FontId>,
    pub size: f32,
    pub width: f32,
    pub height: f32,
    pub color: Option<glam::Vec4>,
    pub texture: Option<Texture>,
    pub visible: bool,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            text: String::from(""),
            font_id: None,
            size: 0.0,
            width: 0.0,
            height: 0.0,
            color: None,
            texture: None,
            visible: true,
        }
    }
}

impl Label {
    pub fn new(text: String, size: f32) -> Label {
        Label {
            text,
            size,
            ..Default::default()
        }
    }

    pub fn new_with_font(text: String, font_id: FontId, size: f32) -> Label {
        Label {
            text,
            font_id: Some(font_id),
            size,
            ..Default::default()
        }
    }
}
