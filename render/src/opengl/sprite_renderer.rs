use crate::opengl::shader::Shader;
use crate::texture::TextureType;
use crate::Texture;
use gl::types::*;
use log::warn;
use std::mem;
use std::ptr;

struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    texture_coords: [f32; 2],
    // color: [f32; 3],
}

fn vertex(position: [f32; 2], color: [f32; 3], texture_coords: [f32; 2]) -> Vertex {
    Vertex {
        position,
        color,
        texture_coords,
    }
}

pub struct SpriteRenderer {
    quad_vao: u32,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut quad_vao: u32 = 0;

        let vertices: [Vertex; 4] = [
            vertex([1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0]), // 0
            vertex([1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0]), // 1
            vertex([0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0]), // 2
            vertex([0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0]), // 3
        ];

        let indices: [u32; 6] = [
            0, 1, 3, //
            1, 2, 3,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut quad_vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(quad_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&vertices) as GLsizeiptr,
                mem::transmute(&vertices[0]),
                gl::STATIC_DRAW,
            );
            //
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                mem::size_of_val(&indices) as GLsizeiptr,
                mem::transmute(&indices[0]),
                gl::STATIC_DRAW,
            );

            // Position
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            // Color
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (2 * mem::size_of::<GLfloat>()) as _,
            );
            gl::EnableVertexAttribArray(1);
            // texture_coords
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (5 * mem::size_of::<GLfloat>()) as _,
            );
            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self { quad_vao }
    }

    pub fn draw_sprite(
        &self,
        texture: &Texture,
        position: glam::Vec2,
        size: glam::Vec2,
        rotate: f32,
        color: glam::Vec3,
        shader: &Shader,
    ) {
        shader.use_program();

        let mut model = glam::Mat4::IDENTITY;
        model *= glam::Mat4::from_translation(glam::vec3(position.x, position.y, 0.0));

        // model *= glam::Mat4::from_translation(glam::vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model *= glam::Mat4::from_rotation_z(rotate.to_radians());
        // model *= glam::Mat4::from_translation(glam::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        model *= glam::Mat4::from_scale(glam::vec3(size.x, size.y, 1.0));

        shader.set_matrix4(&"model", &model, false);
        // Todo: reconsider the color
        // shader.set_vector3f(&"spriteColor", &color, false);

        if let TextureType::OpenGL(texture) = &texture.texture_type {
            unsafe { gl::ActiveTexture(gl::TEXTURE0) };
            texture.bind();
        } else {
            warn!("Where is my desired OpenGL Texture");
        }

        unsafe {
            gl::BindVertexArray(self.quad_vao);

            // Todo: Update Buffer
            // let vertices = [0];
            // let vbo: u32 = 0;
            // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // gl::BufferSubData(
            //     gl::ARRAY_BUFFER,
            //     0,
            //     mem::size_of_val(&vertices) as GLsizeiptr,
            //     mem::transmute(&vertices[0]),
            // );

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
