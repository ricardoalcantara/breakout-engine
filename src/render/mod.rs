pub mod render2d_data;
pub mod render2d_pipeline;
pub mod renderer;
pub mod subtexture;
pub mod texture;
pub mod vertex;

use std::rc::Rc;

use self::{texture::Texture, vertex::Vertex};
use crate::{font::Font, shapes::rectangle::Rect};

pub struct RenderQuad {
    pub size: glam::Vec2,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub center_origin: bool,
    pub color: glam::Vec4,
}

impl RenderQuad {
    pub fn raw_vertices(&self) -> [Vertex; 4] {
        [
            Vertex::default(),
            Vertex::default(),
            Vertex::default(),
            Vertex::default(),
        ]
    }
}

pub struct RenderText<'a> {
    pub text: &'a str,
    pub font: Rc<Font>,
    pub size: u32,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub color: glam::Vec4,
}

pub struct RenderTexture {
    pub texture: Rc<Texture>,
    pub rect: Option<Rect>,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub center_origin: bool,
    pub color: glam::Vec4,
}

impl RenderTexture {
    pub fn raw_vertices(&self) -> [Vertex; 4] {
        [
            Vertex::default(),
            Vertex::default(),
            Vertex::default(),
            Vertex::default(),
        ]
    }
}

pub struct RenderVertices {
    pub texture: Option<Rc<Texture>>,
    pub vertices: [glam::Vec3; 4],
    pub color: glam::Vec4,
    pub texture_coords: [glam::Vec2; 4],
}

impl RenderVertices {
    pub fn raw_vertices(&self, tex_index: u32) -> [Vertex; 4] {
        [
            Vertex {
                position: self.vertices[0],
                color: self.color,
                texture_coords: self.texture_coords[0],
                tex_index,
            },
            Vertex {
                position: self.vertices[1],
                color: self.color,
                texture_coords: self.texture_coords[1],
                tex_index,
            },
            Vertex {
                position: self.vertices[2],
                color: self.color,
                texture_coords: self.texture_coords[2],
                tex_index,
            },
            Vertex {
                position: self.vertices[3],
                color: self.color,
                texture_coords: self.texture_coords[3],
                tex_index,
            },
        ]
    }
}
