use crate::helper::{compile_shader, link_program, VERTEX_DATA};
use crate::shader::Shader;
use crate::sprite_renderer::SpriteRenderer;
use crate::texture::Texture;
use gl::types::*;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::ffi::CStr;
use std::mem;
use std::ptr;
use winit::event::*;

pub struct State {
    shader: Shader,
    texture: Texture,
    sprite_renderer: SpriteRenderer,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub fn new(window: &ContextWrapper<PossiblyCurrent, glutin::window::Window>) -> Self {
        gl::load_with(|symbol| window.get_proc_address(symbol));

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };

        println!("OpenGL version {}", version);

        let vs_src = std::fs::read_to_string("shaders/gl_shader.vert")
            .expect("Something went wrong reading vs_src");
        let fs_src = std::fs::read_to_string("shaders/gl_shader.frag")
            .expect("Something went wrong reading fs_src");

        let projection = glam::Mat4::orthographic_rh_gl(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);
        let shader = Shader::compile(&vs_src, &fs_src, None);

        shader.use_program();
        shader.set_integer(&"image", 0, false);
        shader.set_matrix4(&"projection", &projection, false);

        let mut texture = Texture::new();
        texture.internal_format = gl::RGBA;
        texture.image_format = gl::RGBA;
        texture.generate("assets/awesomeface.png");
        let sprite_renderer = SpriteRenderer::new();

        Self {
            shader,
            texture,
            sprite_renderer,
        }
    }

    pub fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {}

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), ()> {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Draw a triangle from the 3 vertices

            self.sprite_renderer.draw_sprite(
                &self.texture,
                glam::vec2(200.0, 200.0),
                Some(glam::vec2(300.0, 400.0)),
                Some(45.0),
                Some(glam::vec3(0.0, 1.0, 0.0)),
                &self.shader,
            )
        }
        Ok(())
    }
}
