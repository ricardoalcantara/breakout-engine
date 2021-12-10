use crate::texture::Texture;

pub enum RenderAPI {
    OpenGL,
    WGPU,
}

pub trait Renderer2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {}

    fn clean_color(&self) {}
    fn draw_texture(
        &mut self,
        _texture: &Texture,
        _position: glam::Vec2,
        _size: glam::Vec2,
        _rotate: f32,
        _color: glam::Vec3,
    ) {
    }
}
