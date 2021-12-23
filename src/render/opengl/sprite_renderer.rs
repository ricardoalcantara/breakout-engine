use super::shader::Shader;
use super::vertex::{vertex, vertex_format, Vertex};
use crate::render::texture::{Texture, TextureType};
use crate::shapes::rectangle::Rect;
use gl::types::*;
use log::warn;
use std::mem;
use std::ptr;

pub struct SpriteRenderer {
    quad_vao: u32,
    quad_vbo: u32,
    vertices: [Vertex; 4],
}

impl SpriteRenderer {
    pub fn new() -> Self {
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut quad_vao: u32 = 0;

        let vertices: [Vertex; 4] = [
            vertex([1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0]), // 0 // Top Right
            vertex([1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0]), // 1 // Bottom Right
            vertex([0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0]), // 2 // Bottom Left
            vertex([0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0]), // 3 // Top Left
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
                mem::size_of::<vertex_format::Float32x2>() as _, // offset Position
            );
            gl::EnableVertexAttribArray(1);
            // texture_coords
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (mem::size_of::<vertex_format::Float32x2>()
                    + mem::size_of::<vertex_format::Float32x3>()) as _, // offset Position + Color
            );
            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            quad_vao,
            quad_vbo: vbo,
            vertices,
        }
    }

    pub fn draw_sprite(
        &self,
        texture: &Texture,
        rect: Option<Rect>,
        position: glam::Vec2,
        scale: glam::Vec2,
        rotate: f32,
        color: glam::Vec3,
        shader: &Shader,
    ) {
        shader.use_program();

        let mut model = glam::Mat4::IDENTITY;
        // let mut model = glam::Mat4::orthographic_rh(0.0, 512.0, 512.0, 0.0, -1.0, 1.0);
        model *= glam::Mat4::from_translation(glam::vec3(position.x, position.y, 0.0));
        model *=
            glam::Mat4::from_scale(glam::vec3(texture.width as f32, texture.height as f32, 1.0));
        model *= glam::Mat4::from_scale(glam::vec3(scale.x, scale.y, 1.0));

        model *= glam::Mat4::from_rotation_z(rotate.to_radians());

        shader.set_matrix4(&"model", &model);

        let mut vertices = self.vertices;

        for vertex in &mut vertices {
            vertex.color = [color.x, color.y, color.z];
        }

        if let Some(rect) = rect {
            let width = texture.width as f32;
            let height = texture.height as f32;
            vertices[0].texture_coords = [
                (rect.x + rect.width) / width,
                (rect.y + rect.height) / height,
            ]; // Top Right
            vertices[1].texture_coords = [rect.right() / width, rect.y / height]; // Bottom Right
            vertices[2].texture_coords = [rect.x / width, rect.y / height]; // Bottom Left
            vertices[3].texture_coords = [rect.x / width, rect.bottom() / height];
            // Top Left
        }

        if let TextureType::OpenGL(texture) = &texture.texture_type {
            unsafe { gl::ActiveTexture(gl::TEXTURE0) };
            texture.bind();
        } else {
            warn!("Where is my desired OpenGL Texture");
        }

        unsafe {
            gl::BindVertexArray(self.quad_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                mem::size_of_val(&vertices) as GLsizeiptr,
                mem::transmute(&vertices[0]),
            );

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
