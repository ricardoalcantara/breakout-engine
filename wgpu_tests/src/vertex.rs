// main.rs
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
    pub texture_coords: glam::Vec2
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },

            ],
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

pub const TOP_LEFT_QUAD: &[Vertex] = &[
    Vertex { position: glam::const_vec3!([ 1.0, 0.0, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 0.0]) }, // TOP RIGHT
    Vertex { position: glam::const_vec3!([ 0.0, 0.0, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 0.0]) }, // TOP LEFT
    Vertex { position: glam::const_vec3!([ 0.0, 1.0, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 1.0]) }, // BOTTOM LEFT
    Vertex { position: glam::const_vec3!([ 1.0, 1.0, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 1.0]) }, // BOTTOM RIGHT
];

pub const CENTER_QUAD: &[Vertex] = &[
    Vertex { position: glam::const_vec3!([ 0.5,-0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 0.0]) }, // TOP RIGHT
    Vertex { position: glam::const_vec3!([-0.5,-0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 0.0]) }, // TOP LEFT
    Vertex { position: glam::const_vec3!([-0.5, 0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([0.0, 1.0]) }, // BOTTOM LEFT
    Vertex { position: glam::const_vec3!([ 0.5, 0.5, 0.0]), color: glam::const_vec3!([1.0, 1.0, 1.0]), texture_coords: glam::const_vec2!([1.0, 1.0]) }, // BOTTOM RIGHT
];

pub const VERTEX: &[Vertex] = TOP_LEFT_QUAD;

#[rustfmt::skip] 
pub const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];
