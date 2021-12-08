#[cfg(feature = "opengl")]
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use texture::Texture;
#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "default")]
pub mod wgpu;

pub mod texture;

pub struct InternalWindow {
    #[cfg(feature = "opengl")]
    window: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
}

impl InternalWindow {
    pub fn window(&self) -> &Window {
        self.window.window()
    }

    pub fn swap_buffers(&self) {
        self.window.swap_buffers().unwrap()
    }
}

pub trait Render2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {}

    fn input(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn clean_color(&self) {}
    fn draw_texture(
        &mut self,
        _texture: &dyn Texture,
        _position: glam::Vec2,
        _size: glam::Vec2,
        _rotate: f32,
        _color: glam::Vec3,
    ) {
    }
}

pub fn build_window() -> (InternalWindow, winit::event_loop::EventLoop<()>) {
    #[cfg(feature = "opengl")]
    return opengl::build_window();
}
