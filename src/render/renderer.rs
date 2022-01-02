use super::texture::Texture;
use crate::{error::BreakoutResult, shapes::rectangle::Rect};
use image::DynamicImage;

pub enum RenderAPI {
    OpenGL,
    WGPU,
}

pub struct RenderQuad {
    pub size: glam::Vec2,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub color: glam::Vec4,
}

pub struct RenderTexture<'a> {
    pub texture: &'a Texture,
    pub rect: Option<Rect>,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub center_origin: bool,
    pub color: glam::Vec4,
}

pub trait Renderer2D {
    fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>);
    fn generate_texture(&self, img: DynamicImage) -> BreakoutResult<Texture>;
    fn clear_color(&self, _color: glam::Vec3);
    fn begin_draw(&mut self, camera: Option<glam::Mat4>);
    fn end_draw(&mut self);
    fn draw_quad(&mut self, _quad: RenderQuad);
    fn draw_texture(&mut self, _texture: RenderTexture);
}
