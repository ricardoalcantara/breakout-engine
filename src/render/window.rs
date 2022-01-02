use crate::{
    error::BreakoutResult,
    render::{renderer::Renderer2D, RenderAPI},
};
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};

use super::opengl::{self, renderer2d::OpenGLRenderer2D};

pub(crate) enum WindowProvider {
    Glutin(ContextWrapper<PossiblyCurrent, glutin::window::Window>),
}

pub struct MyWindow {
    window_provider: WindowProvider,
    pub event_loop: Option<winit::event_loop::EventLoop<()>>,
    render_api: RenderAPI,
}

impl MyWindow {
    pub fn build(window_builder: winit::window::WindowBuilder, render_api: RenderAPI) -> Self {
        let (window_provider, event_loop) = match render_api {
            RenderAPI::OpenGL => {
                let (window, event_loop) = opengl::build_window(window_builder);

                (WindowProvider::Glutin(window), event_loop)
            }
            RenderAPI::WGPU => todo!(),
        };

        Self {
            window_provider,
            event_loop: Some(event_loop),
            render_api,
        }
    }

    pub fn create_renderer_2d(&self) -> BreakoutResult<impl Renderer2D> {
        match self.render_api {
            RenderAPI::OpenGL => {
                let WindowProvider::Glutin(window) = &self.window_provider;
                OpenGLRenderer2D::new(window)
            }
            RenderAPI::WGPU => todo!(),
        }
    }

    pub fn window(&self) -> &Window {
        match &self.window_provider {
            WindowProvider::Glutin(window) => window.window(),
        }
    }

    pub fn swap_buffers(&self) {
        match &self.window_provider {
            WindowProvider::Glutin(window) => window.swap_buffers().unwrap(),
        };
    }
}
