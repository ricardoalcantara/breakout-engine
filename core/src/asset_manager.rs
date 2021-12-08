use render::texture::Texture;

pub struct AssetManager {
    pub(crate) texture: Option<Box<dyn Texture>>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self { texture: None }
    }

    pub fn load_sprite(&mut self, path: &str) {
        let mut texture = render::opengl::texture::OpenGLTexture::new();
        texture.generate(path);

        self.texture = Some(Box::new(texture));
    }
}
