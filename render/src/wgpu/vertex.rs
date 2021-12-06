use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
        // wgpu::VertexBufferLayout {
        //     array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        //     step_mode: wgpu::VertexStepMode::Vertex,
        //     attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        // }
    }
}

const TRIANGLE: &[Vertex; 3] = &[
    Vertex {
        position: [0.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
];

const RECTANGLE: &[Vertex; 4] = &[
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
];
const RECTANGLE_INDICES: &[u16] = &[0, 1, 2, 2, 3, 0, /* padding */ 0];

pub struct Primitive {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_vertices: u32,
    pub num_indices: u32,
}

impl Primitive {
    pub fn new_triangle(device: &wgpu::Device) -> Self {
        Primitive::from_vertices(device, TRIANGLE)
    }

    pub fn new_rectangle(device: &wgpu::Device) -> Self {
        Primitive::from_vertices_with_indices(device, RECTANGLE, RECTANGLE_INDICES)
    }

    pub fn from_vertices(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = vertices.len() as u32;

        Self {
            vertex_buffer,
            num_vertices,
            index_buffer: None,
            num_indices: 0,
        }
    }

    pub fn from_vertices_with_indices(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = vertices.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        Self {
            vertex_buffer,
            num_vertices,
            index_buffer: Some(index_buffer),
            num_indices,
        }
    }

    pub fn get_buffer(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.slice(..)
    }

    pub fn get_index_buffer(&self) -> Option<wgpu::BufferSlice> {
        if let Some(index_buffer) = &self.index_buffer {
            Some(index_buffer.slice(..))
        } else {
            None
        }
    }
}
