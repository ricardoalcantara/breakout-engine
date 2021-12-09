use render::texture::{Texture, TextureType};

pub struct AssetManager {
    pub(crate) texture: Option<Texture>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self { texture: None }
    }

    pub fn load_sprite(&mut self, path: &str) {
        let mut opengl_texture = render::opengl::texture::OpenGLTexture::new();
        opengl_texture.generate(path);
        let texture = Texture {
            texture_type: TextureType::OpenGL(opengl_texture),
        };

        self.texture = Some(texture);
    }
}
