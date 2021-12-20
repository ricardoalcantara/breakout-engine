use crate::{
    audio::{Audio, AudioSettings},
    render::texture::Texture,
};
use image::DynamicImage;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TextureId(i32);

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AudioId(i32);

pub struct AssetManager {
    texture_id_count: i32,
    audio_id_count: i32,
    preload_textures: HashMap<TextureId, DynamicImage>,
    textures: HashMap<TextureId, Texture>,
    audios: HashMap<AudioId, Audio>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self {
            texture_id_count: 0,
            audio_id_count: 0,
            preload_textures: HashMap::new(),
            textures: HashMap::new(),
            audios: HashMap::new(),
        }
    }

    pub fn load_sprite(&mut self, path: &str) -> TextureId {
        self.texture_id_count += 1;
        let id = TextureId(self.texture_id_count);
        // Todo: BeakoutResult
        let img = image::open(path).unwrap();
        self.preload_textures.insert(id.clone(), img);

        id
    }

    pub fn take_preload_textures(&mut self) -> HashMap<TextureId, DynamicImage> {
        self.preload_textures.drain().collect()
    }

    pub(crate) fn add_texture(&mut self, id: TextureId, texture: Texture) -> TextureId {
        self.textures.insert(id, texture);
        TextureId(0)
    }

    pub fn get_texture(&self, id: &TextureId) -> &Texture {
        &self.textures[id]
    }

    pub fn load_audio(&mut self, path: &str, settings: Option<AudioSettings>) -> AudioId {
        self.audio_id_count += 1;
        let id = AudioId(self.audio_id_count);
        // Todo: BeakoutResult
        match Audio::load(path) {
            Ok(mut audio) => {
                audio.settings = settings;
                self.audios.insert(id.clone(), audio);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        id
    }

    pub fn get_audio(&self, id: &AudioId) -> &Audio {
        &self.audios[id]
    }
}
