use std::collections::HashMap;

use crate::{core::asset_manager::TextureId, shapes::rectangle::Rect};

use super::SubTexture;

pub struct AnimatedSprite {
    pub total_animation_time: f32,
    pub current_animation: String,
    pub current_frame: usize,
    pub animations: HashMap<String, Animation>,
    pub change_to: Option<String>,
}

impl Default for AnimatedSprite {
    fn default() -> Self {
        Self {
            total_animation_time: 0.0,
            current_animation: "".to_owned(),
            current_frame: 0,
            animations: HashMap::new(),
            change_to: None,
        }
    }
}

pub struct Animation {
    pub length: f32,
    pub key_frames: Vec<KeyFrame>,
}

// Todo: for now it will only work for sprites xD
pub struct KeyFrame {
    pub texture_id: Option<TextureId>,
    pub sub_texture: Option<SubTexture>,
    pub time: f32,
    pub effect: Option<String>,
}

impl Default for KeyFrame {
    fn default() -> Self {
        Self {
            texture_id: None,
            sub_texture: None,
            time: 0.0,
            effect: None,
        }
    }
}
// Todo: Keyframe should have actions
