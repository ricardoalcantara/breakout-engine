use crate::helper::{compile_shader, link_program, VERTEX_DATA};
use crate::shader::Shader;
use gl::types::*;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::ffi::CStr;
use std::mem;
use std::ptr;
use winit::event::*;

pub struct State {}

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

        let shader = Shader::compile(&vs_src, &fs_src, None);

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&VERTEX_DATA[0]),
                gl::STATIC_DRAW,
            );

            // Use shader program
            shader.use_program();
            gl::BindFragDataLocation(shader.id, 0, b"out_color\0".as_ptr() as *const _);

            // Specify the layout of the vertex data
            let pos_attr = gl::GetAttribLocation(shader.id, b"position\0".as_ptr() as *const _);
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
        }
        Self {}
    }

    pub fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {}

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), ()> {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}
