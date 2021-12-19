use std::collections::HashMap;

use image::DynamicImage;
use render::texture::Texture;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TextureId(i32);

pub struct AssetManager {
    texture_id_count: i32,
    preload_textures: HashMap<TextureId, DynamicImage>,
    textures: HashMap<TextureId, Texture>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self {
            texture_id_count: 0,
            preload_textures: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_sprite(&mut self, path: &str) -> TextureId {
        self.texture_id_count += 1;
        let id = TextureId(self.texture_id_count);
        let img = image::open(path).unwrap();
        self.preload_textures.insert(id.clone(), img);

        id
    }

    pub fn get_preload_textures(&mut self) -> HashMap<TextureId, DynamicImage> {
        self.preload_textures.drain().collect()
    }

    pub(crate) fn add_texture(&mut self, id: TextureId, texture: Texture) -> TextureId {
        self.textures.insert(id, texture);
        TextureId(0)
    }

    pub fn get_texture(&self, id: &TextureId) -> &Texture {
        &self.textures[id]
    }
}
