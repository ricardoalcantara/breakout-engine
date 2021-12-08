use crate::opengl::shader::Shader;
use crate::Texture;
use gl::types::*;
use std::mem;
use std::ptr;

pub struct SpriteRenderer {
    quad_vao: u32,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        let mut vbo: u32 = 0;
        let mut quad_vao: u32 = 0;

        let vertices: [GLfloat; 24] = [
            // pos      // tex
            0.0, 1.0, 0.0, 1.0, //
            1.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, //
            //
            0.0, 1.0, 0.0, 1.0, //
            1.0, 1.0, 1.0, 1.0, //
            1.0, 0.0, 1.0, 0.0, //
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut quad_vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&vertices) as GLsizeiptr,
                mem::transmute(&vertices[0]),
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(quad_vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * mem::size_of::<GLfloat>()) as GLsizei,
                ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self { quad_vao }
    }

    pub fn draw_sprite(
        &self,
        texture: &dyn Texture,
        position: glam::Vec2,
        size: glam::Vec2,
        rotate: f32,
        color: glam::Vec3,
        shader: &Shader,
    ) {
        shader.use_program();

        let mut model = glam::Mat4::IDENTITY;
        model *= glam::Mat4::from_translation(glam::vec3(position.x, position.y, 0.0));

        model *= glam::Mat4::from_translation(glam::vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model *= glam::Mat4::from_rotation_z(rotate.to_radians());
        model *= glam::Mat4::from_translation(glam::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        model *= glam::Mat4::from_scale(glam::vec3(size.x, size.y, 1.0));

        shader.set_matrix4(&"model", &model, false);
        shader.set_vector3f(&"spriteColor", &color, false);

        unsafe { gl::ActiveTexture(gl::TEXTURE0) };
        texture.bind();

        unsafe {
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}
