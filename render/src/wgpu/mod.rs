use crate::InternalWindow;

pub mod camera;
pub mod pipeline;
pub mod state;
pub mod texture;
pub mod vertex;

pub fn build_window() -> (InternalWindow, winit::event_loop::EventLoop<()>) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new();
    let window = window.build(&event_loop).unwrap();
    // let mut state: State = pollster::block_on(State::new(&window));

    todo!();
    // (Win { window }, event_loop)
}
