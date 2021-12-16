use image::DynamicImage;
use shapes::rectangle::Rectangle;

use crate::texture::Texture;

pub enum RenderAPI {
    OpenGL,
    WGPU,
}

pub trait Renderer2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {}
    fn generate_texture(&self, img: DynamicImage) -> Texture;
    fn clean_color(&self) {}
    fn draw_texture(
        &mut self,
        _texture: Option<&Texture>,
        _rect: Option<Rectangle>,
        _position: glam::Vec2,
        _scale: glam::Vec2,
        _rotate: f32,
        _color: glam::Vec3,
    ) {
    }
}
