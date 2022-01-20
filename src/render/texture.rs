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

#[derive(Clone, Copy)]
pub struct SubTexture {
    pub region: Rect,
    pub texture_size: glam::Vec2,
    pub(crate) texture_coords: Option<[glam::Vec2; 4]>,
}

impl SubTexture {
    pub fn new(region: Rect) -> SubTexture {
        SubTexture {
            region,
            texture_size: glam::Vec2::ZERO,
            texture_coords: None,
        }
    }

    pub fn new_with_texture_size(region: Rect, width: f32, height: f32) -> SubTexture {
        let mut sub_texture = SubTexture::new(region);
        sub_texture.texture_size.x = width;
        sub_texture.texture_size.y = height;
        sub_texture.update_texture_coords();

        sub_texture
    }

    pub fn from_texture(region: Rect, texture: &Texture) -> SubTexture {
        let mut sub_texture = SubTexture::new(region);
        sub_texture.texture_size.x = texture.width as f32;
        sub_texture.texture_size.y = texture.height as f32;
        sub_texture.update_texture_coords();

        sub_texture
    }

    pub(crate) fn update_texture_coords_with_region(&mut self, region: Rect) {
        self.region = region;
        self.update_texture_coords();
    }

    pub(crate) fn update_texture_coords_with_texture_size(&mut self, texture_size: glam::Vec2) {
        self.texture_size = texture_size;
        self.update_texture_coords();
    }

    pub(crate) fn update_texture_coords(&mut self) {
        let width = self.texture_size.x;
        let height = self.texture_size.y;
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
