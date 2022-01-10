use renderer::RenderAPI;
use window::MyWindow;

pub mod opengl;

pub mod renderer;
pub mod texture;
pub mod vertex;
pub mod window;

pub fn build_window(
    window_builder: winit::window::WindowBuilder,
    render_api: RenderAPI,
) -> MyWindow {
    MyWindow::build(window_builder, render_api)
}
