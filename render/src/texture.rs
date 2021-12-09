use crate::opengl::texture::OpenGLTexture;

// Todo pub(crate)
pub enum TextureType {
    OpenGL(OpenGLTexture),
    WGPU,
}

// Todo pub(crate)
pub struct Texture {
    pub texture_type: TextureType,
}
