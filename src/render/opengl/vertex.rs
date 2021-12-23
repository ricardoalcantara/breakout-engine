pub(crate) mod vertex_format {
    pub(crate) type Float32x2 = [f32; 2];
    pub(crate) type Float32x3 = [f32; 3];
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Vertex {
    pub(crate) position: vertex_format::Float32x2,
    pub(crate) color: vertex_format::Float32x3,
    pub(crate) texture_coords: vertex_format::Float32x2,
}

pub(crate) fn vertex(
    position: vertex_format::Float32x2,
    color: vertex_format::Float32x3,
    texture_coords: vertex_format::Float32x2,
) -> Vertex {
    Vertex {
        position,
        color,
        texture_coords,
    }
}
