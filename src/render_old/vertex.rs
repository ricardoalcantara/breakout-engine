pub(crate) enum QuadOrigin {
    TopLeft,
    Center,
}

pub(crate) const TOP_LEFT_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([1.0, 1.0, 0.0, 1.0]),
    glam::const_vec4!([1.0, 0.0, 0.0, 1.0]),
    glam::const_vec4!([0.0, 0.0, 0.0, 1.0]),
    glam::const_vec4!([0.0, 1.0, 0.0, 1.0]),
];

pub(crate) const CENTER_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([0.5, 0.5, 0.0, 1.0]),
    glam::const_vec4!([0.5, -0.5, 0.0, 1.0]),
    glam::const_vec4!([-0.5, -0.5, 0.0, 1.0]),
    glam::const_vec4!([-0.5, 0.5, 0.0, 1.0]),
];

pub(crate) const TEXTURE_COORDS: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 1.0]),
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
];

pub(crate) const TEXTURE_COORDS_FLIPPED_X: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 1.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
];

pub(crate) const TEXTURE_COORDS_FLIPPED_Y: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([1.0, 1.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([0.0, 0.0]),
];

pub(crate) const TEXTURE_COORDS_FLIPPED_X_Y: [glam::Vec2; 4] = [
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([1.0, 1.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([1.0, 0.0]),
];

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
