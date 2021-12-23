use crate::{
    error::BreakoutResult,
    render::{opengl::render2d_pipeline::Render2dPipeline, renderer::Renderer2D, texture::Texture},
    shapes::rectangle::Rect,
};
use glutin::{ContextWrapper, PossiblyCurrent};
use log::info;
use std::ffi::CStr;

use super::texture::OpenGLTexture;

pub struct OpenGLRenderer2D {
    render2d_pipeline: Render2dPipeline,
}

impl OpenGLRenderer2D {
    pub fn new(
        window: &ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    ) -> BreakoutResult<Self> {
        gl::load_with(|symbol| window.get_proc_address(symbol));

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };

        info!("OpenGL version {}", version);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let render2d_pipeline = Render2dPipeline::new();

        Ok(Self { render2d_pipeline })
    }
}

impl Renderer2D for OpenGLRenderer2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(0, 0, _new_size.width as _, _new_size.height as _);
        }
    }

    fn generate_texture(&self, img: image::DynamicImage) -> BreakoutResult<Texture> {
        OpenGLTexture::generate_texture(img)
    }

    fn clear_color(&self, color: glam::Vec3) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn begin_draw(&mut self) {
        self.render2d_pipeline.begin_batch();
    }

    fn end_draw(&mut self) {
        self.render2d_pipeline.end_batch();
        self.render2d_pipeline.flush();
    }

    fn draw_quad(
        &mut self,
        size: glam::Vec2,
        position: glam::Vec2,
        scale: glam::Vec2,
        rotate: f32,
        color: glam::Vec4,
    ) {
        self.render2d_pipeline
            .draw_quad(size, position, scale, rotate, color);
    }

    fn draw_texture(
        &mut self,
        texture: &Texture,
        rect: Option<Rect>,
        position: glam::Vec2,
        scale: glam::Vec2,
        rotate: f32,
        color: glam::Vec4,
    ) {
        self.render2d_pipeline
            .draw_texture(texture, rect, position, scale, rotate, color);
    }
}
