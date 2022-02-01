#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub color: glam::Vec4,
    pub texture_coords: glam::Vec2,
    pub tex_index: u32,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x2, 3 => Uint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Vertex::ATTRIBS,
        }
    }
}

#[rustfmt::skip] 
// pub const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: glam::vec3(0.0, 0.5, 0.0),
//         color: glam::vec3(1.0, 0.0, 0.0),
//     },
//     Vertex {
//         position: glam::vec3(-0.5, -0.5, 0.0),
//         color: glam::vec3(0.0, 1.0, 0.0),
//     },
//     Vertex {
//         position: glam::vec3(0.5, -0.5, 0.0),
//         color: glam::vec3(0.0, 0.0, 1.0),
//     },
// ];

// pub const VERTICES: &[Vertex] = &[
//     Vertex { position: glam::const_vec3!([ 0.5, 0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 0.0]) }, // TOP RIGHT
//     Vertex { position: glam::const_vec3!([-0.5, 0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 0.0]) }, // TOP LEFT
//     Vertex { position: glam::const_vec3!([-0.5,-0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 1.0]) }, // BOTTOM LEFT
//     Vertex { position: glam::const_vec3!([ 0.5,-0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 1.0]) }, // BOTTOM RIGHT
// ];

pub enum QuadOrigin {
    TopLeft,
    Center,
}

pub const TOP_LEFT_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([1.0, 0.0, 0.0, 1.0]), // TOP RIGHT
    glam::const_vec4!([0.0, 0.0, 0.0, 1.0]), // TOP LEFT
    glam::const_vec4!([0.0, 1.0, 0.0, 1.0]), // BOTTOM LEFT
    glam::const_vec4!([1.0, 1.0, 0.0, 1.0]), // BOTTOM RIGHT
];

pub const CENTER_QUAD: [glam::Vec4; 4] = [
    glam::const_vec4!([0.5, -0.5, 0.0, 1.0]),  // TOP RIGHT
    glam::const_vec4!([-0.5, -0.5, 0.0, 1.0]), // TOP LEFT
    glam::const_vec4!([-0.5, 0.5, 0.0, 1.0]),  // BOTTOM LEFT
    glam::const_vec4!([0.5, 0.5, 0.0, 1.0]),   // BOTTOM RIGHT
];

pub(crate) const TEXTURE_COORDS: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([1.0, 1.0]),
];

// TODO
pub(crate) const TEXTURE_COORDS_FLIPPED_X: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([1.0, 1.0]),
];

// TODO
pub(crate) const TEXTURE_COORDS_FLIPPED_Y: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([1.0, 1.0]),
];

// TODO
pub(crate) const TEXTURE_COORDS_FLIPPED_X_Y: [glam::Vec2; 4] = [
    glam::const_vec2!([1.0, 0.0]),
    glam::const_vec2!([0.0, 0.0]),
    glam::const_vec2!([0.0, 1.0]),
    glam::const_vec2!([1.0, 1.0]),
];
