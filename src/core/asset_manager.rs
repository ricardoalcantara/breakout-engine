use crate::{
    audio::{Audio, AudioSettings},
    error::{BreakoutError, BreakoutResult},
    render::{font::Font, texture::Texture},
};
use image::DynamicImage;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TextureId(i32);

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AudioId(i32);

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct FontId(i32);

pub struct AssetManager {
    // Textures
    texture_id_count: i32,
    preload_textures: HashMap<TextureId, DynamicImage>,
    textures: HashMap<TextureId, Texture>,
    // Audios
    audio_id_count: i32,
    audios: HashMap<AudioId, Audio>,
    // Fonts
    font_id_count: i32,
    fonts: HashMap<FontId, Font>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self {
            texture_id_count: 0,
            preload_textures: HashMap::new(),
            textures: HashMap::new(),
            audio_id_count: 0,
            audios: HashMap::new(),
            font_id_count: 0,
            fonts: HashMap::new(),
        }
    }
}

impl AssetManager {
    pub fn load_texture(&mut self, path: &str) -> BreakoutResult<TextureId> {
        let texture = image::open(path).map_err(BreakoutError::ImageError)?;

        self.texture_id_count += 1;
        let id = TextureId(self.texture_id_count);
        self.preload_textures.insert(id.clone(), texture);

        Ok(id)
    }

    pub(crate) fn take_preload_textures(&mut self) -> HashMap<TextureId, DynamicImage> {
        self.preload_textures.drain().collect()
    }

    pub(crate) fn add_texture(&mut self, id: TextureId, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn get_texture(&self, id: &TextureId) -> &Texture {
        &self.textures[id]
    }
}

impl AssetManager {
    pub fn load_audio(
        &mut self,
        path: &str,
        settings: Option<AudioSettings>,
    ) -> BreakoutResult<AudioId> {
        let mut audio = Audio::load(path).map_err(BreakoutError::IOError)?;
        audio.settings = settings;

        self.audio_id_count += 1;
        let id = AudioId(self.audio_id_count);
        self.audios.insert(id.clone(), audio);

        Ok(id)
    }

    pub fn get_audio(&self, id: &AudioId) -> &Audio {
        &self.audios[id]
    }
}

impl AssetManager {
    pub fn load_font(&mut self, path: &str) -> BreakoutResult<FontId> {
        let font = Font::new(path)?;

        self.font_id_count += 1;
        let id = FontId(self.font_id_count);
        self.fonts.insert(id.clone(), font);

        Ok(id)
    }

    pub fn get_font(&self, id: &FontId) -> &Font {
        &self.fonts[id]
    }
}
