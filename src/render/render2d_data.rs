use super::vertex::{QuadOrigin, Vertex, CENTER_QUAD, TOP_LEFT_QUAD};
use crate::render::texture::Texture;
use crate::shapes::rectangle::Rect;
use std::rc::Rc;

pub const MAX_QUAD_COUNT: usize = 10000;
pub const MAX_VERTEX_COUNT: usize = MAX_QUAD_COUNT * 4;
pub const MAX_INDEX_COUNT: usize = MAX_QUAD_COUNT * 6;
pub const MAX_TEXTURE_COUNT: usize = 32;

pub struct Render2dData {
    quad_count: i32,
    vertices: Box<[Vertex]>,

    texture_slot_index: usize,
    texture_slots: [Option<Rc<Texture>>; MAX_TEXTURE_COUNT],
    texture_max_texture_count: usize,
}

impl Render2dData {
    pub fn new(texture_max_texture_count: usize, white_texture: Texture) -> Render2dData {
        assert!(
            texture_max_texture_count <= MAX_TEXTURE_COUNT,
            "texture_max_texture_count {} is higher than MAX_TEXTURE_COUNT {}",
            texture_max_texture_count,
            MAX_TEXTURE_COUNT
        );

        let mut texture_slots = [None; MAX_TEXTURE_COUNT];
        texture_slots[0] = Some(Rc::new(white_texture));

        Render2dData {
            quad_count: 0,
            vertices: vec![Vertex::default(); MAX_VERTEX_COUNT].into_boxed_slice(),

            texture_slot_index: 1,
            texture_slots,
            texture_max_texture_count,
        }
    }

    pub fn can_add_quad(&self) -> bool {
        self.quad_count < MAX_QUAD_COUNT as i32
    }

    // TODO Caution
    pub fn can_add_quad_with_texture(&self, append_texture: &Rc<Texture>) -> bool {
        self.quad_count < MAX_QUAD_COUNT as i32
            && (self.texture_slots.iter().any(|texture| {
                if let Some(tex) = texture {
                    Rc::ptr_eq(tex, append_texture)
                } else {
                    false
                }
            }) || self.texture_slot_index < self.texture_max_texture_count)
    }

    // TODO Caution
    pub fn append_texture(&mut self, append_texture: &Rc<Texture>) -> u32 {
        assert!(
            self.texture_slot_index < self.texture_max_texture_count,
            "It's not possible to append more textures than the driver supports"
        );
        for (i, texture) in self.texture_slots.iter().enumerate() {
            if let Some(texture) = texture {
                if Rc::ptr_eq(texture, append_texture) {
                    return i as u32;
                }
            }
        }

        let texture_index = self.texture_slot_index;
        self.texture_slots[texture_index] = Some(append_texture.clone());

        self.texture_slot_index += 1;

        texture_index as u32
    }

    pub fn add_vertices(
        &mut self,
        vertices: &[glam::Vec3; 4],
        color: glam::Vec4,
        texture_coords: &[glam::Vec2; 4],
        tex_index: u32,
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

    pub fn add_quad(
        &mut self,
        position: glam::Vec2,
        texture_size: glam::Vec2,
        sub_tex_rect: Option<Rect>,
        scale: glam::Vec2,
        rotate: f32,
        color: glam::Vec4,
        origin: QuadOrigin,
        tex_index: u32,
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

    pub fn reset(&mut self) {
        self.quad_count = 0;
        self.texture_slot_index = 1;
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices[0..self.vertices_count() as usize]
    }

    pub fn vertices_count(&self) -> i32 {
        self.quad_count * 4
    }

    pub fn indices_count(&self) -> i32 {
        self.quad_count * 6
    }

    pub fn bind_textures(&self) {
        todo!()
        // unsafe {
        //     for (i, t) in self.texture_slots[0..self.texture_max_texture_count as usize]
        //         .iter()
        //         .enumerate()
        //     {
        //         // gl::BindTextureUnit(i as GLuint, *t);
        //         gl::ActiveTexture(gl::TEXTURE0 + i as u32);
        //         gl::BindTexture(gl::TEXTURE_2D, *t);
        //     }
        // }
        // check_gl_ok().unwrap();
    }
}
