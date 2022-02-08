use std::rc::Rc;

use log::warn;
use wgpu::util::DeviceExt;

use super::{
    render2d_data::{Render2dData, RenderItem, MAX_INDEX_COUNT, MAX_QUAD_COUNT},
    renderer::RenderContext,
    texture::Texture,
    vertex::{QuadOrigin, Vertex},
    RenderQuad, RenderTexture, RenderVertices,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub projection: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new(projection: &glam::Mat4) -> Self {
        Self {
            projection: projection.to_cols_array_2d(),
        }
    }
}

pub struct Render2DPineline {
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    render_data: Render2dData,

    default_camera: glam::Mat4,
    camera_buffer: wgpu::Buffer,

    camera_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Render2DPineline {
    pub fn new(
        width: u32,
        height: u32,
        max_textures: i32,
        device: &Rc<wgpu::Device>,
        queue: &Rc<wgpu::Queue>,
        config: &wgpu::SurfaceConfiguration,
    ) -> Render2DPineline {
        let vs_src = include_str!("../../shaders/render2d_shader.vert");
        let fs_src = include_str!("../../shaders/render2d_shader.frag");
        // let vs_src = std::fs::read_to_string("shaders/render2d_shader.vert").unwrap();
        // let fs_src = std::fs::read_to_string("shaders/render2d_shader.frag").unwrap();
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

        let mut texture_bind_group_layout_entries = Vec::new();
        texture_bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        for i in 0..max_textures {
            texture_bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32 + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            })
        }

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &texture_bind_group_layout_entries,
                label: Some("texture_bind_group_layout"),
            });

        let white_texture = Texture::from_color([255, 255, 255, 255], device, queue);

        let camera_bind_group_layout =
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
                label: Some("camera_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
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
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let render_data = Render2dData::new(max_textures as usize, white_texture);

        let vertices: [Vertex; MAX_QUAD_COUNT] = [Vertex::default(); MAX_QUAD_COUNT];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let mut indices: [u16; MAX_INDEX_COUNT] = [0u16; MAX_INDEX_COUNT];
        let mut offset = 0;
        for i in (0..MAX_INDEX_COUNT).step_by(6) {
            indices[i + 0] = 0 + offset;
            indices[i + 1] = 1 + offset;
            indices[i + 2] = 2 + offset;
            indices[i + 3] = 0 + offset;
            indices[i + 4] = 2 + offset;
            indices[i + 5] = 3 + offset;

            offset += 4;
        }

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        let camera_uniform = Uniforms::new(&default_camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Render2DPineline {
            device: device.clone(),
            queue: queue.clone(),

            render_pipeline,
            vertex_buffer,
            index_buffer,

            render_data,

            default_camera,
            camera_buffer,

            camera_bind_group,
            texture_bind_group_layout,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        self.default_camera();
    }

    pub fn set_camera(&self, camera: glam::Mat4) {
        let camera_uniform = Uniforms::new(&camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn default_camera(&self) {
        let camera_uniform = Uniforms::new(&self.default_camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn begin_batch(&mut self) {
        self.render_data.begin_batch();
    }

    pub fn draw<'a>(&'a mut self, render_context: &mut RenderContext) {
        let render_steps = self
            .render_data
            .get_render_vertices_and_textures(&self.device, &self.texture_bind_group_layout);

        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&render_steps.buffer_vertices),
        );

        {
            let mut render_pass =
                render_context
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &render_context.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            for texture_bind in &render_steps.texture_binds {
                let indices_from = (texture_bind.from / 4) * 6;
                let indices_to = (texture_bind.to / 4) * 6;

                render_pass.set_bind_group(0, &texture_bind.texture_bind_group, &[]);
                render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                // TOOD remove cast later
                render_pass.draw_indexed(indices_from as u32..indices_to as u32, 0, 0..1);
            }
        }

        // let texture_bind_group = self
        //     .render_data
        //     .bind_textures(&self.texture_bind_group_layout, &self.device);

        // render_pass.set_pipeline(&self.render_pipeline);

        // render_pass.set_bind_group(0, &texture_bind_group, &[]);
        // render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // // TOOD remove cast later
        // render_pass.draw_indexed(0..self.render_data.indices_count() as u32, 0, 0..1);
    }

    pub fn draw_vertices(&mut self, vertices: RenderVertices) {
        self.render_data
            .add_render_item(RenderItem::RenderVertices(vertices));
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        self.render_data
            .add_render_item(RenderItem::RenderQuad(quad));
    }

    pub fn draw_texture(&mut self, texture: RenderTexture) {
        self.render_data
            .add_render_item(RenderItem::RenderTexture(texture));
    }
}
