#[cfg(feature = "opengl")]
use crate::{
    opengl::{self, render2d::OpenGLRender2D},
    renderer::Renderer2D,
    RenderAPI,
};
#[cfg(feature = "opengl")]
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};

pub(crate) enum WindowProvider {
    #[cfg(feature = "opengl")]
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
            #[cfg(feature = "opengl")]
            RenderAPI::OpenGL => {
                let (window, event_loop) = opengl::build_window(window_builder);

                (WindowProvider::Glutin(window), event_loop)
            }
            #[cfg(feature = "default")]
            RenderAPI::WGPU => todo!(),
            _ => todo!(""),
        };

        Self {
            window_provider,
            event_loop: Some(event_loop),
            render_api,
        }
    }

    pub fn create_renderer_2d(&self) -> impl Renderer2D {
        match self.render_api {
            RenderAPI::OpenGL => {
                let WindowProvider::Glutin(window) = &self.window_provider;
                OpenGLRender2D::new(window)
            }
            #[cfg(feature = "default")]
            RenderAPI::WGPU => todo!(),
            _ => todo!(""),
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
