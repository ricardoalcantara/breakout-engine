use super::texture::Texture;
use crate::{error::BreakoutResult, shapes::rectangle::Rect};
use image::DynamicImage;

pub enum RenderAPI {
    OpenGL,
    WGPU,
}

pub trait Renderer2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {}
    fn generate_texture(&self, img: DynamicImage) -> BreakoutResult<Texture>;
    fn clear_color(&self, _color: glam::Vec3) {}
    fn draw_texture(
        &mut self,
        _texture: Option<&Texture>,
        _rect: Option<Rect>,
        _position: glam::Vec2,
        _scale: glam::Vec2,
        _rotate: f32,
        _color: glam::Vec3,
    ) {
    }
}
