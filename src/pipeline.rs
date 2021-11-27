pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub async fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let (vs_module, fs_module) = {
            let vs_src = include_str!("shader.vert");
            let fs_src = include_str!("shader.frag");
            let mut compiler = shaderc::Compiler::new().unwrap();
            let vs_spirv = compiler
                .compile_into_spirv(
                    vs_src,
                    shaderc::ShaderKind::Vertex,
                    "shader.vert",
                    "main",
                    None,
                )
                .unwrap();
            let fs_spirv = compiler
                .compile_into_spirv(
                    fs_src,
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
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main", // 1.
                buffers: &[],        // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            // continued ...
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
        });

        Self { render_pipeline }
    }

    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline); // 2.
        render_pass.draw(0..3, 0..1); // 3.
    }
}
