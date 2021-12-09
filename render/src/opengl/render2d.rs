use crate::opengl::shader::Shader;
use crate::opengl::sprite_renderer::SpriteRenderer;
use crate::InternalWindow;

use crate::Render2D;
use crate::Texture;
use log::info;
use std::ffi::CStr;
use winit::event::*;
pub struct OpenGLRender2D {
    sprite_shader: Shader,
    sprite_renderer: SpriteRenderer,
}

impl OpenGLRender2D {
    pub fn new(win: &InternalWindow) -> Self {
        let window = &win.window;
        gl::load_with(|symbol| window.get_proc_address(symbol));

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };

        info!("OpenGL version {}", version);

        let vs_src = std::fs::read_to_string("shaders/gl_shader.vert")
            .expect("Something went wrong reading vs_src");
        let fs_src = std::fs::read_to_string("shaders/gl_shader.frag")
            .expect("Something went wrong reading fs_src");

        let projection = glam::Mat4::orthographic_rh_gl(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);
        let shader = Shader::compile(&vs_src, &fs_src, None);

        shader.use_program();
        shader.set_integer(&"image", 0, false);
        shader.set_matrix4(&"projection", &projection, false);

        let sprite_renderer = SpriteRenderer::new();

        Self {
            sprite_shader: shader,
            sprite_renderer,
        }
    }
}

impl Render2D for OpenGLRender2D {
    fn resize(&self, _new_size: winit::dpi::PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(0, 0, _new_size.width as _, _new_size.height as _);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn clean_color(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn draw_texture(
        &mut self,
        texture: &Texture,
        position: glam::Vec2,
        size: glam::Vec2,
        rotate: f32,
        color: glam::Vec3,
    ) {
        self.sprite_renderer.draw_sprite(
            texture,
            position,
            size,
            rotate,
            color,
            &self.sprite_shader,
        );
    }
}
