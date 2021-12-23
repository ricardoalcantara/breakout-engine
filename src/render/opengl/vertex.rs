pub(crate) mod vertex_format {
    pub(crate) type Float32x2 = [f32; 2];
    pub(crate) type Float32x3 = [f32; 3];
    pub(crate) type Float32x4 = [f32; 4];
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Vertex {
    pub(crate) position: vertex_format::Float32x3,
    pub(crate) color: vertex_format::Float32x4,
    pub(crate) texture_coords: vertex_format::Float32x2,
    pub(crate) tex_index: f32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
            texture_coords: [0.0, 0.0],
            tex_index: 0.0,
        }
    }
}

impl Vertex {
    pub(crate) fn new(
        position: vertex_format::Float32x3,
        color: vertex_format::Float32x4,
        texture_coords: vertex_format::Float32x2,
        tex_index: f32,
    ) -> Vertex {
        Vertex {
            position,
            color,
            texture_coords,
            tex_index,
        }
    }
}
