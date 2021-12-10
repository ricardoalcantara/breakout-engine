pub mod camera;
pub mod pipeline;
pub mod state;
pub mod texture;
pub mod vertex;

pub fn build_window() -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new();
    let window = window.build(&event_loop).unwrap();

    (window, event_loop)
}
