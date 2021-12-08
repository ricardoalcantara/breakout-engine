use glutin::{ContextWrapper, PossiblyCurrent};

use crate::InternalWindow;

pub mod render2d;
mod shader;
mod sprite_renderer;
pub mod texture;

pub fn build_window() -> (InternalWindow, winit::event_loop::EventLoop<()>) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let window = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let window: ContextWrapper<PossiblyCurrent, glutin::window::Window> =
        unsafe { window.make_current() }.unwrap();

    (InternalWindow { window }, event_loop)
}
