use super::check_gl_ok;
use super::shader::Shader;
use super::vertex::Vertex;
use crate::render::renderer::{RenderQuad, RenderTexture};
use crate::render::texture::{Texture, TextureType};
use crate::shapes::rectangle::Rect;
use gl::types::*;
use log::warn;
use memoffset::offset_of;
use std::ffi::c_void;
use std::mem;
use std::ptr;

const MAX_QUAD_COUNT: usize = 10000;
const MAX_VERTEX_COUNT: usize = MAX_QUAD_COUNT * 4;
const MAX_INDEX_COUNT: usize = MAX_QUAD_COUNT * 6;
const MAX_TEXTURE_COUNT: usize = 32;

const TOP_LEFT_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([1.0, 1.0, 0.0, 1.0]),
    glam::const_vec4!([1.0, 0.0, 0.0, 1.0]),
    glam::const_vec4!([0.0, 0.0, 0.0, 1.0]),
    glam::const_vec4!([0.0, 1.0, 0.0, 1.0]),
];

const CENTER_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([0.5, 0.5, 0.0, 1.0]),
    glam::const_vec4!([0.5, -0.5, 0.0, 1.0]),
    glam::const_vec4!([-0.5, -0.5, 0.0, 1.0]),
    glam::const_vec4!([-0.5, 0.5, 0.0, 1.0]),
];

pub enum QuadOrigin {
    TopLeft,
    Center,
}

struct Render2dData {
    quad_vao: u32,
    quad_vbo: u32,
    quad_ebo: u32,

    quad_count: i32,
    vertices: Box<[Vertex; MAX_VERTEX_COUNT]>,

    texture_slot_index: usize,
    texture_slots: [u32; MAX_TEXTURE_COUNT],
}

impl Render2dData {
    fn new() -> Render2dData {
        let mut white_texture = 0;
        unsafe {
            gl::GenTextures(1, &mut white_texture);
            gl::BindTexture(gl::TEXTURE_2D, white_texture);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // let data: [u8; 4] = [255, 255, 255, 255];
            let data: u32 = 0xffffffff;
            #[rustfmt::skip]
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::mem::transmute::<*const u32, *const c_void>(&data));
            check_gl_ok().unwrap();
            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        let mut texture_slots = [0; MAX_TEXTURE_COUNT];
        texture_slots[0] = white_texture;

        Render2dData {
            quad_vbo: 0,
            quad_vao: 0,
            quad_ebo: 0,

            quad_count: 0,
            vertices: Box::new([Vertex::default(); MAX_VERTEX_COUNT]),

            texture_slot_index: 1,
            texture_slots,
        }
    }

    fn can_add_quad(&self) -> bool {
        self.quad_count < MAX_QUAD_COUNT as i32
    }

    fn append_texture(&mut self, tex_id: u32) -> f32 {
        for (i, tex_index) in self.texture_slots.iter().enumerate() {
            if *tex_index == tex_id {
                return i as f32;
            }
        }

        let texture_index = self.texture_slot_index;
        self.texture_slots[texture_index] = tex_id;

        self.texture_slot_index += 1;

        texture_index as f32
    }

    fn add_quad(
        &mut self,
        position: glam::Vec2,
        texture_size: glam::Vec2,
        sub_tex_rect: Option<Rect>,
        scale: glam::Vec2,
        rotate: f32,
        color: glam::Vec4,
        origin: QuadOrigin,
        tex_index: f32,
    ) {
        let offset = self.quad_count as usize * 4;
        let render_rect_size = if let Some(r) = sub_tex_rect {
            r.size().into()
        } else {
            texture_size
        };

        let quad = match origin {
            QuadOrigin::TopLeft => &TOP_LEFT_QUAD,
            QuadOrigin::Center => &CENTER_QUAD,
        };

        if rotate == 0.0 {
            self.vertices[offset].position = glam::vec3(
                position.x + (render_rect_size.x * scale.x),
                position.y + (render_rect_size.y * scale.x),
                0.0,
            );
            self.vertices[offset + 1].position =
                glam::vec3(position.x + (render_rect_size.x * scale.x), position.y, 0.0);
            self.vertices[offset + 2].position = glam::vec3(position.x, position.y, 0.0);
            self.vertices[offset + 3].position =
                glam::vec3(position.x, position.y + (render_rect_size.y * scale.x), 0.0);
        } else {
            let transform = glam::Mat4::from_scale_rotation_translation(
                glam::vec3(render_rect_size.x, render_rect_size.y, 1.0)
                    * glam::vec3(scale.x, scale.y, 1.0),
                glam::Quat::from_rotation_z(rotate),
                glam::vec3(position.x, position.y, 1.0),
            );

            self.vertices[offset].position = {
                let tmp = transform * quad[0];
                glam::vec3(tmp.x, tmp.y, tmp.z)
            };
            self.vertices[offset + 1].position = {
                let tmp = transform * quad[1];
                glam::vec3(tmp.x, tmp.y, tmp.z)
            };
            self.vertices[offset + 2].position = {
                let tmp = transform * quad[2];
                glam::vec3(tmp.x, tmp.y, tmp.z)
            };
            self.vertices[offset + 3].position = {
                let tmp = transform * quad[3];
                glam::vec3(tmp.x, tmp.y, tmp.z)
            };
        }

        self.vertices[offset].color = color;
        self.vertices[offset + 1].color = color;
        self.vertices[offset + 2].color = color;
        self.vertices[offset + 3].color = color;

        if let Some(rect) = sub_tex_rect {
            let width = texture_size.x;
            let height = texture_size.y;
            self.vertices[offset].texture_coords = glam::vec2(
                (rect.x + rect.width) / width,
                (rect.y + rect.height) / height,
            ); // Top Right
            self.vertices[offset + 1].texture_coords =
                glam::vec2(rect.right() / width, rect.y / height); // Bottom Right
            self.vertices[offset + 2].texture_coords =
                glam::vec2((rect.x + 0.5) / width, rect.y / height); // Bottom Left
            self.vertices[offset + 3].texture_coords =
                glam::vec2((rect.x + 0.5) / width, rect.bottom() / height); // Top Left
        } else {
            self.vertices[offset].texture_coords = glam::vec2(1.0, 1.0);
            self.vertices[offset + 1].texture_coords = glam::vec2(1.0, 0.0);
            self.vertices[offset + 2].texture_coords = glam::vec2(0.0, 0.0);
            self.vertices[offset + 3].texture_coords = glam::vec2(0.0, 1.0);
        }

        self.vertices[offset].tex_index = tex_index;
        self.vertices[offset + 1].tex_index = tex_index;
        self.vertices[offset + 2].tex_index = tex_index;
        self.vertices[offset + 3].tex_index = tex_index;

        self.quad_count += 1;
    }

    fn reset(&mut self) {
        self.quad_count = 0;
        self.texture_slot_index = 1;
    }

    fn vertices(&self) -> &[Vertex] {
        &self.vertices[0..self.vertices_count() as usize]
    }

    fn vertices_count(&self) -> i32 {
        self.quad_count * 4
    }

    fn indices_count(&self) -> i32 {
        self.quad_count * 6
    }

    fn bind_textures(&self) {
        unsafe {
            for (i, t) in self.texture_slots.iter().enumerate() {
                // gl::BindTextureUnit(i as GLuint, *t);
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, *t);
            }
        }
        check_gl_ok().unwrap();
    }
}

pub struct Render2dPipeline {
    shader: Shader,
    render_data: Render2dData,
    default_camera: glam::Mat4,
}

impl Render2dPipeline {
    pub fn new(width: u32, height: u32) -> Self {
        #[cfg(dev_shader)]
        let vs_src = std::fs::read_to_string("shaders/render2d_shader.vert")
            .expect("Something went wrong reading vs_src");
        #[cfg(dev_shader)]
        let fs_src = std::fs::read_to_string("shaders/render2d_shader.frag")
            .expect("Something went wrong reading fs_src");

        #[cfg(not(dev_shader))]
        let vs_src = include_str!("../../../shaders/render2d_shader.vert");
        #[cfg(not(dev_shader))]
        let fs_src = include_str!("../../../shaders/render2d_shader.frag");

        let default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        let shader = Shader::compile(&vs_src, &fs_src, None);

        shader.use_program();
        let mut textures = [0; MAX_TEXTURE_COUNT];
        for i in 0..MAX_TEXTURE_COUNT {
            textures[i] = i as i32;
        }

        shader.set_integer_vector(&"u_textures", &textures);
        // for i in 0..MAX_TEXTURE_COUNT {
        //     shader.set_integer(&format!("u_texture{}", i), i as i32);
        // }

        shader.set_matrix4(&"projection", &default_camera);

        let mut render_data = Render2dData::new();

        unsafe {
            gl::GenVertexArrays(1, &mut render_data.quad_vao);
            gl::GenBuffers(1, &mut render_data.quad_vbo);
            gl::BindVertexArray(render_data.quad_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, render_data.quad_vbo);
            #[rustfmt::skip]
            gl::BufferData(gl::ARRAY_BUFFER, (MAX_VERTEX_COUNT * mem::size_of::<Vertex>()) as GLsizeiptr, ptr::null(), gl::DYNAMIC_DRAW);
            check_gl_ok().unwrap();
            // Position
            gl::EnableVertexAttribArray(0);
            #[rustfmt::skip]
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, position) as _);
            // Color
            gl::EnableVertexAttribArray(1);
            #[rustfmt::skip]
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, color) as _);
            // texture_coords
            gl::EnableVertexAttribArray(2);
            #[rustfmt::skip]
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, texture_coords) as _);
            // texture_id
            gl::EnableVertexAttribArray(3);
            #[rustfmt::skip]
            gl::VertexAttribPointer(3, 1, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, tex_index) as _);

            // Indices
            let mut indices: [u32; MAX_INDEX_COUNT] = [0u32; MAX_INDEX_COUNT];
            let mut offset = 0;
            for i in (0..MAX_INDEX_COUNT).step_by(6) {
                indices[i + 0] = 0 + offset;
                indices[i + 1] = 1 + offset;
                indices[i + 2] = 3 + offset;
                indices[i + 3] = 1 + offset;
                indices[i + 4] = 2 + offset;
                indices[i + 5] = 3 + offset;

                offset += 4;
            }
            gl::GenBuffers(1, &mut render_data.quad_ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, render_data.quad_ebo);
            #[rustfmt::skip]
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, mem::size_of_val(&indices) as GLsizeiptr, mem::transmute(&indices[0]), gl::STATIC_DRAW);
            check_gl_ok().unwrap();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            shader,
            render_data,
            default_camera,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        self.shader.set_matrix4(&"projection", &self.default_camera);
    }

    pub fn set_camera(&self, camera: glam::Mat4) {
        self.shader.set_matrix4(&"projection", &camera);
    }

    pub fn default_camera(&self) {
        self.shader.set_matrix4(&"projection", &self.default_camera);
    }

    pub fn begin_batch(&mut self) {
        self.render_data.quad_count = 0;
    }

    pub fn end_batch(&self) {
        let vertices = self.render_data.vertices();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.render_data.quad_vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                mem::size_of_val(vertices) as GLsizeiptr,
                if vertices.len() > 0 {
                    mem::transmute(&vertices[0])
                } else {
                    ptr::null()
                },
            );
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.render_data.bind_textures();

            gl::BindVertexArray(self.render_data.quad_vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.render_data.indices_count(),
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }
        self.render_data.reset()
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        self.shader.use_program();

        if !self.render_data.can_add_quad() {
            self.end_batch();
            self.flush();
            self.begin_batch()
        }

        self.render_data.add_quad(
            quad.position,
            quad.size,
            None,
            quad.scale,
            quad.rotate,
            quad.color,
            QuadOrigin::TopLeft,
            0.0,
        );
    }

    pub fn draw_texture(&mut self, render_texture: RenderTexture) {
        let texture = render_texture.texture;

        self.shader.use_program();

        if !self.render_data.can_add_quad() {
            self.end_batch();
            self.flush();
            self.begin_batch()
        }

        let tex_index = if let TextureType::OpenGL(opengl_texture) = &texture.texture_type {
            self.render_data.append_texture(opengl_texture.id)
        } else {
            0.0
        };

        self.render_data.add_quad(
            render_texture.position,
            glam::vec2(texture.width as f32, texture.height as f32),
            render_texture.rect,
            render_texture.scale,
            render_texture.rotate,
            render_texture.color,
            if render_texture.center_origin {
                QuadOrigin::Center
            } else {
                QuadOrigin::TopLeft
            },
            tex_index,
        );
    }
}
