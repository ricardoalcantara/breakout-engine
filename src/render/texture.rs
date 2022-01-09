use crate::shapes::rectangle::Rect;

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

impl Texture {
    pub fn size(&self) -> glam::UVec2 {
        glam::uvec2(self.width, self.height)
    }
}

pub struct SubTexture {
    pub region: Rect,
    pub(crate) texture_coords: Option<[glam::Vec2; 4]>,
}

impl SubTexture {
    pub fn new(region: Rect) -> SubTexture {
        SubTexture {
            region,
            texture_coords: None,
        }
    }

    pub fn from_texture(region: Rect, texture: &Texture) -> SubTexture {
        let mut sub_texture = SubTexture::new(region);
        sub_texture.update_texture_coords(texture);

        sub_texture
    }

    pub(crate) fn update_texture_coords(&mut self, texture: &Texture) {
        let width = texture.width as f32;
        let height = texture.height as f32;
        let mut texture_coords = [glam::Vec2::ZERO; 4];

        texture_coords[0] = glam::vec2(
            (self.region.x + self.region.width) / width,
            (self.region.y + self.region.height) / height,
        ); // Top Right
        texture_coords[1] = glam::vec2(self.region.right() / width, self.region.y / height); // Bottom Right
        texture_coords[2] = glam::vec2((self.region.x + 0.5) / width, self.region.y / height); // Bottom Left
        texture_coords[3] =
            glam::vec2((self.region.x + 0.5) / width, self.region.bottom() / height);
        // Top Left

        self.texture_coords = Some(texture_coords);
    }
}
