use super::opengl::texture::OpenGLTexture;

// TODO pub(crate)
pub enum TextureType {
    OpenGL(OpenGLTexture),
    WGPU,
}

// TODO pub(crate)
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub texture_type: TextureType,
}
