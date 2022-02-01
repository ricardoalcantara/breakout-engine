use crate::{
    error::BreakoutResult,
    render::{
        opengl::render2d_pipeline::Render2dPipeline,
        renderer::{RenderQuad, RenderText, RenderTexture, RenderVertices},
        texture::Texture,
    },
};
use glutin::{ContextWrapper, PossiblyCurrent};
use log::info;
use std::ffi::CStr;

use super::texture::OpenGLTexture;

pub struct OpenGLRenderer2D {
    render2d_pipeline: Render2dPipeline,
    render_size: Option<glam::UVec2>,
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

        Ok(Self {
            render2d_pipeline,
            render_size: None,
        })
    }

    pub fn set_render_size(&mut self, render_size: glam::UVec2) {
        self.render_size = Some(render_size);
        self.render2d_pipeline.resize(render_size.x, render_size.y);
    }

    pub fn render_size(&self) -> Option<glam::UVec2> {
        self.render_size
    }

    pub fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(0, 0, _new_size.width as _, _new_size.height as _);
            if self.render_size.is_none() {
                self.render2d_pipeline
                    .resize(_new_size.width, _new_size.height);
            }
        }
    }

    pub fn generate_texture(&self, img: image::DynamicImage) -> BreakoutResult<Texture> {
        OpenGLTexture::generate_texture(img)
    }

    pub fn clear_color(&self, color: glam::Vec3) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn begin_draw(&mut self, camera: Option<glam::Mat4>) {
        if let Some(camera) = camera {
            self.render2d_pipeline.set_camera(camera);
        } else {
            self.render2d_pipeline.default_camera();
        }
        self.render2d_pipeline.begin_batch();
    }

    pub fn end_draw(&mut self) {
        self.render2d_pipeline.end_batch();
        self.render2d_pipeline.flush();
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        self.render2d_pipeline.draw_quad(quad);
    }

    pub fn draw_texture(&mut self, texture: RenderTexture) {
        self.render2d_pipeline.draw_texture(texture);
    }

    pub fn draw_text(&mut self, _text: RenderText) {
        _text.font.draw_vertices(
            _text.text,
            _text.position,
            _text.size,
            |texture, vertices, texture_coords| {
                self.draw_vertices(RenderVertices {
                    texture: Some(texture),
                    vertices: &vertices,
                    texture_coords,
                    color: _text.color,
                })
            },
        )
    }

    pub fn draw_vertices(&mut self, _vertices: RenderVertices) {
        self.render2d_pipeline.draw_vertices(
            _vertices.vertices,
            _vertices.color,
            _vertices.texture_coords,
            _vertices.texture,
        )
    }
}