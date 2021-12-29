use super::opengl::texture::OpenGLTexture;

pub enum TextureType {
    OpenGL(OpenGLTexture),
    WGPU,
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub texture_type: TextureType,
}
