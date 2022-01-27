// main.rs
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: cgmath::Vector3<f32>,
    pub color: cgmath::Vector3<f32>,
    pub texture_coords: cgmath::Vector2<f32>
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
//         position: cgmath::vec3(0.0, 0.5, 0.0),
//         color: cgmath::vec3(1.0, 0.0, 0.0),
//     },
//     Vertex {
//         position: cgmath::vec3(-0.5, -0.5, 0.0),
//         color: cgmath::vec3(0.0, 1.0, 0.0),
//     },
//     Vertex {
//         position: cgmath::vec3(0.5, -0.5, 0.0),
//         color: cgmath::vec3(0.0, 0.0, 1.0),
//     },
// ];
// main.rs
pub const VERTICES: &[Vertex] = &[
    Vertex { position: cgmath::vec3( 0.5, 0.5, 0.0), color: cgmath::vec3(1.0, 1.0, 1.0), texture_coords: cgmath::vec2(1.0, 0.0) }, // TOP RIGHT
    Vertex { position: cgmath::vec3(-0.5, 0.5, 0.0), color: cgmath::vec3(1.0, 1.0, 1.0), texture_coords: cgmath::vec2(0.0, 0.0) }, // TOP LEFT
    Vertex { position: cgmath::vec3(-0.5,-0.5, 0.0), color: cgmath::vec3(1.0, 1.0, 1.0), texture_coords: cgmath::vec2(0.0, 1.0) }, // BOTTOM LEFT
    Vertex { position: cgmath::vec3( 0.5,-0.5, 0.0), color: cgmath::vec3(1.0, 1.0, 1.0), texture_coords: cgmath::vec2(1.0, 1.0) }, // BOTTOM RIGHT
];

#[rustfmt::skip] 
pub const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];
