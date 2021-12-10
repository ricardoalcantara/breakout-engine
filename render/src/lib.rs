use renderer::RenderAPI;
use texture::Texture;
use window::MyWindow;

#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "default")]
pub mod wgpu;

pub mod renderer;
pub mod texture;
pub mod window;

pub fn build_window(
    window_builder: winit::window::WindowBuilder,
    render_api: RenderAPI,
) -> MyWindow {
    MyWindow::build(window_builder, render_api)
}
