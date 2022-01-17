use super::texture::Texture;
use crate::{error::BreakoutResult, font::Font, shapes::rectangle::Rect};
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
    pub center_origin: bool,
    pub color: glam::Vec4,
}

pub struct RenderText<'a> {
    pub text: &'a str,
    pub font: &'a Font,
    pub size: u32,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
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

pub struct RenderVertices<'a> {
    pub texture: Option<&'a Texture>,
    pub vertices: &'a [glam::Vec3; 4],
    pub color: glam::Vec4,
    pub texture_coords: &'a [glam::Vec2; 4],
}

pub trait Renderer2D {
    fn set_render_size(&mut self, _render_size: glam::UVec2);
    fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>);
    fn generate_texture(&self, img: DynamicImage) -> BreakoutResult<Texture>;
    fn clear_color(&self, _color: glam::Vec3);
    fn begin_draw(&mut self, camera: Option<glam::Mat4>);
    fn end_draw(&mut self);
    fn draw_quad(&mut self, _quad: RenderQuad);
    fn draw_texture(&mut self, _texture: RenderTexture);
    fn draw_text(&mut self, _text: RenderText);
    fn draw_vertices(&mut self, _text: RenderVertices);
}
