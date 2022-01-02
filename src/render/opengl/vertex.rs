#[allow(dead_code)]
pub(crate) mod vertex_format {
    pub(crate) type Float32x2 = [f32; 2];
    pub(crate) type Float32x3 = [f32; 3];
    pub(crate) type Float32x4 = [f32; 4];
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Vertex {
    pub(crate) position: glam::Vec3,
    pub(crate) color: glam::Vec4,
    pub(crate) texture_coords: glam::Vec2,
    pub(crate) tex_index: f32,
}

impl Vertex {
    #[allow(dead_code)]
    pub(crate) fn new(
        position: glam::Vec3,
        color: glam::Vec4,
        texture_coords: glam::Vec2,
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
