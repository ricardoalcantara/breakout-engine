use image::DynamicImage;

use crate::{
    audio::{Audio, AudioSettings},
    error::{BreakoutError, BreakoutResult},
    font::Font,
    render::{renderer::Renderer, texture::Texture},
};
use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

use super::game_window::ReadOnlyRc;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TextureId(i32);

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AudioId(i32);

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct FontId(i32);

struct AutoIncrementId {
    ids: HashMap<String, i32>,
}

impl AutoIncrementId {
    fn new() -> AutoIncrementId {
        AutoIncrementId {
            ids: HashMap::new(),
        }
    }

    fn get_id<T>(&mut self) -> i32 {
        let key = std::any::type_name::<T>().to_string();
        let id = if self.ids.contains_key(&key) {
            self.ids[&key] + 1
        } else {
            0
        };

        self.ids.insert(key, id);
        id
    }
}

pub struct AssetManager {
    auto_increment_id: AutoIncrementId,
    textures: HashMap<TextureId, Rc<Texture>>,
    audios: HashMap<AudioId, Audio>,
    fonts: HashMap<FontId, Rc<Font>>,
    renderer: ReadOnlyRc<Renderer>,
}

impl AssetManager {
    pub(crate) fn new(renderer: ReadOnlyRc<Renderer>) -> Self {
        Self {
            auto_increment_id: AutoIncrementId::new(),
            textures: HashMap::new(),
            audios: HashMap::new(),
            fonts: HashMap::new(),
            renderer,
        }
    }
}

impl AssetManager {
    pub fn load_texture(&mut self, path: &str) -> BreakoutResult<TextureId> {
        // let image = image::open(path).map_err(BreakoutError::ImageError)?;
        let renderer = self.renderer.borrow();
        let mut texture = Texture::from_file(path, renderer.device(), renderer.queue());

        let id = TextureId(self.auto_increment_id.get_id::<TextureId>());
        texture.id = Some(id.0);
        self.textures.insert(id.clone(), Rc::new(texture));

        Ok(id)
    }

    pub fn get_texture(&self, id: &TextureId) -> &Rc<Texture> {
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

        let id = AudioId(self.auto_increment_id.get_id::<AudioId>());
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

        let id = FontId(self.auto_increment_id.get_id::<FontId>());
        self.fonts.insert(id.clone(), Rc::new(font));

        Ok(id)
    }

    pub fn get_font(&self, id: &FontId) -> &Rc<Font> {
        &self.fonts[id]
    }

    pub fn get_font_with_size<F>(
        &mut self,
        id: &FontId,
        size: u32,
        get_texture: F,
    ) -> BreakoutResult<&Font>
    where
        F: FnOnce(DynamicImage) -> Texture,
    {
        if !self.fonts[&id].has_size(size) {
            // TODO unsafe
            let mut font = self.fonts.remove(id).unwrap();
            (*Rc::get_mut(&mut font).unwrap()).build_with_size(size, get_texture)?;
            self.fonts.insert(id.clone(), font);
        }

        Ok(&self.fonts[id])
    }
}
