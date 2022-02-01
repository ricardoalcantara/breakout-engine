use super::check_gl_ok;
use super::shader::Shader;
use crate::render::renderer::{RenderQuad, RenderTexture};
use crate::render::texture::{Texture, TextureType};
use crate::render::vertex::{QuadOrigin, Vertex, CENTER_QUAD, TOP_LEFT_QUAD};
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

struct Render2dData {
    quad_vao: u32,
    quad_vbo: u32,
    quad_ebo: u32,

    quad_count: i32,
    vertices: Box<[Vertex]>,

    texture_slot_index: usize,
    texture_slots: [u32; MAX_TEXTURE_COUNT],
    texture_max_texture_count: usize,
}

impl Render2dData {
    fn new(texture_max_texture_count: usize) -> Render2dData {
        assert!(
            texture_max_texture_count <= MAX_TEXTURE_COUNT,
            "texture_max_texture_count {} is higher than MAX_TEXTURE_COUNT {}",
            texture_max_texture_count,
            MAX_TEXTURE_COUNT
        );

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
            vertices: vec![Vertex::default(); MAX_VERTEX_COUNT].into_boxed_slice(),

            texture_slot_index: 1,
            texture_slots,
            texture_max_texture_count,
        }
    }

    fn can_add_quad(&self) -> bool {
        self.quad_count < MAX_QUAD_COUNT as i32
    }

    fn can_add_quad_with_texture(&self, tex_id: &u32) -> bool {
        self.quad_count < MAX_QUAD_COUNT as i32
            && (self.texture_slots.contains(tex_id)
                || self.texture_slot_index < self.texture_max_texture_count)
    }

    fn append_texture(&mut self, tex_id: u32) -> f32 {
        assert!(
            self.texture_slot_index < self.texture_max_texture_count,
            "It's not possible to append more textures than the driver supports"
        );
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

    fn add_vertices(
        &mut self,
        vertices: &[glam::Vec3; 4],
        color: glam::Vec4,
        texture_coords: &[glam::Vec2; 4],
        tex_index: f32,
    ) {
        let offset = self.quad_count as usize * 4;

        self.vertices[offset].position = vertices[0];
        self.vertices[offset + 1].position = vertices[1];
        self.vertices[offset + 2].position = vertices[2];
        self.vertices[offset + 3].position = vertices[3];

        self.vertices[offset].color = color;
        self.vertices[offset + 1].color = color;
        self.vertices[offset + 2].color = color;
        self.vertices[offset + 3].color = color;

        self.vertices[offset].texture_coords = texture_coords[0];
        self.vertices[offset + 1].texture_coords = texture_coords[1];
        self.vertices[offset + 2].texture_coords = texture_coords[2];
        self.vertices[offset + 3].texture_coords = texture_coords[3];

        self.vertices[offset].tex_index = tex_index;
        self.vertices[offset + 1].tex_index = tex_index;
        self.vertices[offset + 2].tex_index = tex_index;
        self.vertices[offset + 3].tex_index = tex_index;

        self.quad_count += 1;
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

        let transform = if rotate == 0.0 {
            glam::Mat4::from_translation(position.extend(0.0))
                * glam::Mat4::from_scale(render_rect_size.extend(0.0) * scale.extend(0.0))
        } else {
            glam::Mat4::from_scale_rotation_translation(
                render_rect_size.extend(0.0) * scale.extend(0.0),
                glam::Quat::from_rotation_z(rotate),
                position.extend(0.0),
            )
        };

        self.vertices[offset].position = (transform * quad[0]).truncate();
        self.vertices[offset + 1].position = (transform * quad[1]).truncate();
        self.vertices[offset + 2].position = (transform * quad[2]).truncate();
        self.vertices[offset + 3].position = (transform * quad[3]).truncate();

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
            for (i, t) in self.texture_slots[0..self.texture_max_texture_count as usize]
                .iter()
                .enumerate()
            {
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
    pub fn new(width: u32, height: u32, max_textures: i32) -> Self {
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

        let fs = str::replace(&fs_src, "$MAX_TEXTURE_COUNT", &max_textures.to_string());
        let default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        let shader = Shader::compile(&vs_src, &fs, None);

        shader.use_program();
        let mut textures = [0; MAX_TEXTURE_COUNT];
        for i in 0..MAX_TEXTURE_COUNT {
            textures[i] = i as i32;
        }

        shader.set_integer_vector(&"u_textures", &textures[0..max_textures as usize]);
        // for i in 0..max_textures {
        //     shader.set_integer(&format!("u_texture{}", i), i as i32);
        // }

        shader.set_matrix4(&"projection", &default_camera);

        let mut render_data = Render2dData::new(max_textures as usize);

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
            self.shader.use_program();

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

    pub fn draw_vertices(
        &mut self,
        vertices: &[glam::Vec3; 4],
        color: glam::Vec4,
        texture_coords: &[glam::Vec2; 4],
        texture: Option<&Texture>,
    ) {
        let tex_id = if let Some(texture) = texture {
            if let TextureType::OpenGL(opengl_texture) = &texture.texture_type {
                opengl_texture.id
            } else {
                warn!("It would have a TextureType::OpenGL texture_type");
                self.render_data.texture_slots[0]
            }
        } else {
            self.render_data.texture_slots[0]
        };

        if !self.render_data.can_add_quad_with_texture(&tex_id) {
            self.end_batch();
            self.flush();
            self.begin_batch()
        }

        let tex_index = self.render_data.append_texture(tex_id);
        self.render_data
            .add_vertices(vertices, color, texture_coords, tex_index)
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
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
            if quad.center_origin {
                QuadOrigin::Center
            } else {
                QuadOrigin::TopLeft
            },
            0.0,
        );
    }

    pub fn draw_texture(&mut self, render_texture: RenderTexture) {
        let texture = render_texture.texture;

        let tex_id = if let TextureType::OpenGL(opengl_texture) = &texture.texture_type {
            opengl_texture.id
        } else {
            warn!("It would have a TextureType::OpenGL texture_type");
            self.render_data.texture_slots[0]
        };

        if !self.render_data.can_add_quad_with_texture(&tex_id) {
            self.end_batch();
            self.flush();
            self.begin_batch()
        }

        let tex_index = self.render_data.append_texture(tex_id);

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