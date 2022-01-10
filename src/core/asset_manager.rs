use image::DynamicImage;

use crate::{
    audio::{Audio, AudioSettings},
    error::{BreakoutError, BreakoutResult},
    font::Font,
    render::{renderer::Renderer2D, texture::Texture},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Hash, PartialEq, Eq, Clone)]
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
    textures: HashMap<TextureId, Texture>,
    audios: HashMap<AudioId, Audio>,
    fonts: HashMap<FontId, Font>,
    renderer: Rc<RefCell<dyn Renderer2D>>,
}

impl AssetManager {
    pub(crate) fn new<R>(renderer: Rc<RefCell<R>>) -> Self
    where
        R: Renderer2D + 'static,
    {
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
        let image = image::open(path).map_err(BreakoutError::ImageError)?;
        let texture = self.renderer.borrow().generate_texture(image)?;

        let id = TextureId(self.auto_increment_id.get_id::<TextureId>());
        self.textures.insert(id.clone(), texture);

        Ok(id)
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
        self.fonts.insert(id.clone(), font);

        Ok(id)
    }

    pub fn get_font(&self, id: &FontId) -> &Font {
        &self.fonts[id]
    }

    pub fn get_font_with_size<F>(
        &mut self,
        id: &FontId,
        size: u32,
        get_texture: F,
    ) -> BreakoutResult<&Font>
    where
        F: FnOnce(DynamicImage) -> BreakoutResult<Texture>,
    {
        if !self.fonts[&id].has_size(size) {
            let mut font = self.fonts.remove(id).unwrap();
            font.build_with_size(size, get_texture)?;
            self.fonts.insert(id.clone(), font);
        }

        Ok(&self.fonts[id])
    }
}
