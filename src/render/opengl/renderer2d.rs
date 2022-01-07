use crate::{
    error::BreakoutResult,
    render::{
        opengl::render2d_pipeline::Render2dPipeline,
        renderer::{RenderQuad, RenderText, RenderTexture, Renderer2D},
        texture::Texture,
    },
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

        let (version, max_texture_image_units) = unsafe {
            let mut max_texture_image_units = 0;
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_texture_image_units);
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            (String::from_utf8(data).unwrap(), max_texture_image_units)
        };

        info!("OpenGL version {}", version);
        info!("Max Texture Image Units {}", max_texture_image_units);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let s = window.window().inner_size();
        let render2d_pipeline = Render2dPipeline::new(s.width, s.height, max_texture_image_units);

        Ok(Self { render2d_pipeline })
    }
}

impl Renderer2D for OpenGLRenderer2D {
    fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(0, 0, _new_size.width as _, _new_size.height as _);
            self.render2d_pipeline
                .resize(_new_size.width, _new_size.height)
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

    fn begin_draw(&mut self, camera: Option<glam::Mat4>) {
        if let Some(camera) = camera {
            self.render2d_pipeline.set_camera(camera);
        } else {
            self.render2d_pipeline.default_camera();
        }
        self.render2d_pipeline.begin_batch();
    }

    fn end_draw(&mut self) {
        self.render2d_pipeline.end_batch();
        self.render2d_pipeline.flush();
    }

    fn draw_quad(&mut self, quad: RenderQuad) {
        self.render2d_pipeline.draw_quad(quad);
    }

    fn draw_texture(&mut self, texture: RenderTexture) {
        self.render2d_pipeline.draw_texture(texture);
    }

    fn draw_text(&mut self, _text: RenderText) {
        _text.font.draw(
            _text.text,
            _text.position,
            _text.size,
            |texture, position, rect| {
                self.draw_texture(RenderTexture {
                    texture,
                    rect: Some(rect),
                    position: _text.position + position,
                    scale: _text.scale,
                    rotate: 0.0,
                    center_origin: false,
                    color: _text.color,
                })
            },
        )
    }
}
