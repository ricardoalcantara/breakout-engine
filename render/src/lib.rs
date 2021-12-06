#[cfg(feature = "opengl")]
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "default")]
pub mod wgpu;

pub struct Win {
    #[cfg(feature = "opengl")]
    window: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
}

impl Win {
    pub fn window(&self) -> &Window {
        self.window.window()
    }

    pub fn swap_buffers(&self) {
        self.window.swap_buffers().unwrap()
    }
}

pub fn build_window() -> (Win, winit::event_loop::EventLoop<()>) {
    #[cfg(feature = "opengl")]
    return opengl::build_window();
}
