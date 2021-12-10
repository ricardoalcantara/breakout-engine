#[cfg(feature = "opengl")]
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use texture::Texture;
#[cfg(feature = "opengl")]
pub mod opengl;
pub mod texture;
#[cfg(feature = "default")]
pub mod wgpu;

pub enum RenderAPI {
    OpenGL,
    WGPU,
}

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

pub fn build_window(render_api: RenderAPI) -> (InternalWindow, winit::event_loop::EventLoop<()>) {
    match render_api {
        #[cfg(feature = "opengl")]
        RenderAPI::OpenGL => opengl::build_window(),
        #[cfg(feature = "default")]
        RenderAPI::WGPU => todo!(),
        _ => todo!(""),
    }
}
