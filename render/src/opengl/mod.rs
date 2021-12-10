use glutin::{ContextWrapper, PossiblyCurrent};

pub mod render2d;
pub mod texture;

mod shader;
mod sprite_renderer;

pub fn build_window(
    window_builder: winit::window::WindowBuilder,
) -> (
    ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    winit::event_loop::EventLoop<()>,
) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let window: ContextWrapper<PossiblyCurrent, glutin::window::Window> =
        unsafe { window.make_current() }.unwrap();

    (window, event_loop)
}
