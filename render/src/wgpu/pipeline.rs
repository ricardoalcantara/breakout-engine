use wgpu::util::DeviceExt;

use crate::wgpu::{
    camera::Camera,
    texture::Texture,
    vertex::{Primitive, Vertex},
};

pub struct CameraUniform {
    pub uniform_data: CameraData,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl CameraUniform {
    fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        })
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let uniform_bind_group_layout = CameraUniform::create_bind_group_layout(device);
        let uniform = CameraData::new();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            uniform_data: uniform,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn from_matrix(matrix: glam::Mat4, device: &wgpu::Device) -> Self {
        let mut camera_uniform = Self::new(device);
        camera_uniform.uniform_data.set_matrix(matrix);
        camera_uniform
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraData {
    matrix: [[f32; 4]; 4],
}

impl CameraData {
    fn new() -> Self {
        Self {
            matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn set_matrix(&mut self, matrix: glam::Mat4) {
        self.matrix = matrix.to_cols_array_2d();
    }
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    camera_uniform: CameraUniform,
}

impl Pipeline {
    pub async fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let (vs_module, fs_module) = {
            let vs_src = std::fs::read_to_string("shaders/shader.vert")
                .expect("Something went wrong reading src/shader.vert");
            let fs_src = std::fs::read_to_string("shaders/shader.frag")
                .expect("Something went wrong reading src/shader.frag");

            let mut compiler = shaderc::Compiler::new().unwrap();
            let vs_spirv = compiler
                .compile_into_spirv(
                    &vs_src,
                    shaderc::ShaderKind::Vertex,
                    "shader.vert",
                    "main",
                    None,
                )
                .unwrap();
            let fs_spirv = compiler
                .compile_into_spirv(
                    &fs_src,
                    shaderc::ShaderKind::Fragment,
                    "shader.frag",
                    "main",
                    None,
                )
                .unwrap();
            let vs_data = wgpu::util::make_spirv(vs_spirv.as_binary_u8());
            let fs_data = wgpu::util::make_spirv(fs_spirv.as_binary_u8());
            let vs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader"),
                source: vs_data,
            });
            let fs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Fragment Shader"),
                source: fs_data,
            });
            (vs_module, fs_module)
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    // Todo: cache or make it static
                    &Texture::create_bind_group_layout(device),
                    &CameraUniform::create_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            // continued ...
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let camera_uniform = CameraUniform::new(device);

        Self {
            render_pipeline,
            camera_uniform,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        data: &[FrameData<'a>],
    ) {
        render_pass.set_pipeline(&self.render_pipeline);

        for item in data {
            let uniform = if let Some(camera) = item.camera {
                // Todo: Move to Camera Uniform
                &camera.camera_uniform.uniform_bind_group
            } else {
                // Todo: Default Camera
                &self.camera_uniform.uniform_bind_group
            };

            render_pass.set_bind_group(1, uniform, &[]);
            render_pass.set_bind_group(0, &item.texture.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, item.primitive.get_buffer());
            // Todo: Aqui mesmo:
            if let Some(index_buffer) = item.primitive.get_index_buffer() {
                render_pass.set_index_buffer(index_buffer, wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..item.primitive.num_indices, 0, 0..1);
            } else {
                render_pass.draw(0..item.primitive.num_vertices, 0..1);
            }
        }

        // render_pass.set_bind_group(1, &self.uniform.uniform_bind_group, &[]);
        // for (index, primitive) in primitives.iter().enumerate() {
        //     render_pass.set_bind_group(0, &textures[index].diffuse_bind_group, &[]);
        //     render_pass.set_vertex_buffer(0, primitive.get_buffer());
        //     // Todo: Aqui mesmo:
        //     if let Some(index_buffer) = primitive.get_index_buffer() {
        //         render_pass.set_index_buffer(index_buffer, wgpu::IndexFormat::Uint16);
        //         render_pass.draw_indexed(0..primitive.num_indices, 0, 0..1);
        //     } else {
        //         render_pass.draw(0..primitive.num_vertices, 0..1);
        //     }
        // }
    }
}

pub struct FrameData<'a> {
    pub primitive: &'a Primitive,
    pub texture: &'a Texture,
    pub camera: Option<&'a Camera>,
}
