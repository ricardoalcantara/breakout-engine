use std::rc::Rc;

use log::warn;
use wgpu::util::DeviceExt;

use super::{
    render2d_data::{Render2dData, MAX_INDEX_COUNT},
    texture::Texture,
    vertex::{QuadOrigin, Vertex},
    RenderQuad, RenderTexture,
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
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    render_data: Render2dData,

    default_camera: glam::Mat4,
    camera_buffer: wgpu::Buffer,

    camera_bind_group: wgpu::BindGroup,
    textures_bind_group: wgpu::BindGroup,
}

impl Render2DPineline {
    pub fn new(
        width: u32,
        height: u32,
        max_textures: i32,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
    ) -> Render2DPineline {
        // let vs_src = include_str!("../../shaders/render2d_shader.vert");
        // let fs_src = include_str!("../../shaders/render2d_shader.frag");
        // TODO include_str
        let vs_src = std::fs::read_to_string("shaders/render2d_shader.vert").unwrap();
        let fs_src = std::fs::read_to_string("shaders/render2d_shader.frag").unwrap();
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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            // SamplerBindingType::Comparison is only for TextureSampleType::Depth
                            // SamplerBindingType::Filtering if the sample_type of the texture is:
                            //     TextureSampleType::Float { filterable: true }
                            // Otherwise you'll get an error.
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

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
                    blend: Some(wgpu::BlendState::REPLACE),
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

        let vertices: [Vertex; 0] = [];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
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
        let num_indices = indices.len() as u32;

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

        let textures_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[],
            label: Some("diffuse_bind_group"),
        });

        let white_texture = Texture::from_color([255, 255, 255, 255], device, queue);

        let render_data = Render2dData::new(max_textures as usize, white_texture);

        Render2DPineline {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,

            render_data,

            default_camera,
            camera_buffer,

            camera_bind_group,
            textures_bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &wgpu::Queue) {
        self.default_camera =
            glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        self.default_camera(queue);
    }

    pub fn set_camera(&self, camera: glam::Mat4, queue: &wgpu::Queue) {
        let camera_uniform = Uniforms::new(&camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn default_camera(&self, queue: &wgpu::Queue) {
        let camera_uniform = Uniforms::new(&self.default_camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn begin_batch(&mut self) {
        self.render_data.reset();
    }

    pub fn end_batch(&self, queue: &wgpu::Queue) {
        let vertices = self.render_data.vertices();
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
    }

    pub fn flush<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        // TODO
        self.render_data.bind_textures();

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.textures_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // TOOD remove cast later
        render_pass.draw_indexed(0..self.render_data.indices_count() as u32, 0, 0..1);

        // TODO: RENDER
        // self.render();

        // gl::DrawElements(
        //     gl::TRIANGLES,
        //     self.render_data.indices_count(),
        //     gl::UNSIGNED_INT,
        //     ptr::null(),
        // );
        // gl::BindVertexArray(0);
        self.render_data.reset()
    }

    pub fn draw_vertices<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        vertices: &[glam::Vec3; 4],
        color: glam::Vec4,
        texture_coords: &[glam::Vec2; 4],
        texture: Option<&Rc<Texture>>,
    ) {
        let mut tex_index = 0;
        if let Some(texture) = texture {
            if !self.render_data.can_add_quad_with_texture(texture) {
                self.end_batch(queue);
                self.flush(render_pass);
                self.begin_batch()
            }
            tex_index = self.render_data.append_texture(texture);
        } else {
            if !self.render_data.can_add_quad() {
                self.end_batch(queue);
                self.flush(render_pass);
                self.begin_batch()
            }
        }

        self.render_data
            .add_vertices(vertices, color, texture_coords, tex_index)
    }

    pub fn draw_quad<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        quad: RenderQuad,
    ) {
        if !self.render_data.can_add_quad() {
            self.end_batch(queue);
            self.flush(render_pass);
            self.begin_batch()
        }

        self.render_data.add_quad(
            quad.position,
            quad.size,
            None,
            quad.scale,
            quad.rotate,
            quad.color,
            if quad.center_origin {
                QuadOrigin::Center
            } else {
                QuadOrigin::TopLeft
            },
            0,
        );
    }

    pub fn draw_texture<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        render_texture: RenderTexture,
    ) {
        let texture = render_texture.texture;

        if !self.render_data.can_add_quad_with_texture(&texture) {
            self.end_batch(queue);
            self.flush(render_pass);
            self.begin_batch()
        }

        let tex_index = self.render_data.append_texture(texture);

        self.render_data.add_quad(
            render_texture.position,
            glam::vec2(texture.width as f32, texture.height as f32),
            render_texture.rect,
            render_texture.scale,
            render_texture.rotate,
            render_texture.color,
            if render_texture.center_origin {
                QuadOrigin::Center
            } else {
                QuadOrigin::TopLeft
            },
            tex_index,
        );
    }

    // pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     render_pass.set_pipeline(&self.render_pipeline); // 2.

    //     render_pass.set_bind_group(0, &self.textures_bind_group, &[]);
    //     render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
    //     render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.
    // }
}
