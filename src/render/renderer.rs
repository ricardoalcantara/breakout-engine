use super::{
    render2d_pipeline::Render2DPineline, RenderQuad, RenderText, RenderTexture, RenderVertices,
};
use log::info;
use std::iter;
use winit::window::Window;

pub struct RenderContext {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render2d_pipeline: Render2DPineline,
    clear_color: wgpu::Color,

    render_context: Option<RenderContext>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let size = window.inner_size();

        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let power_preference = wgpu::util::power_preference_from_env()
            .unwrap_or(wgpu::PowerPreference::HighPerformance);
        let force_fallback_adapter = std::env::var("WGPU_FORCE_FALLBACK")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter,
            })
            .await
            .unwrap();

        let adapter_info = adapter.get_info();
        info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: adapter.limits(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
        info!("Limits {:#?}", device.limits());

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            // TODO VSYNC
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let render2d_pipeline =
            Render2DPineline::new(size.width, size.height, 32, &device, &config, &queue);

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render2d_pipeline,
            clear_color,

            render_context: None,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn reconfigure(&mut self) {
        self.resize(self.size);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn clear_color(&mut self, color: glam::Vec3) {
        self.clear_color = wgpu::Color {
            r: color.x as f64 / 255.0,
            g: color.x as f64 / 255.0,
            b: color.x as f64 / 255.0,
            a: 1.0,
        }
    }

    pub fn begin_draw(&mut self, camera: Option<glam::Mat4>) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.render_context = Some(RenderContext {
            output,
            view,
            encoder,
        });

        if let Some(camera) = camera {
            self.render2d_pipeline.set_camera(camera, &self.queue);
        } else {
            self.render2d_pipeline.default_camera(&self.queue);
        }
        self.render2d_pipeline.begin_batch();
    }

    pub fn end_draw(&mut self) {
        // TODO no_unwrap
        let mut render_context = self.render_context.take().unwrap();

        self.render2d_pipeline.end_batch(&self.queue);
        self.render2d_pipeline.flush(&mut render_context);

        self.queue
            .submit(iter::once(render_context.encoder.finish()));
        render_context.output.present();
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        if let Some(render_context) = &mut self.render_context {
            self.render2d_pipeline
                .draw_quad(render_context, &self.queue, quad);
        } else {
            panic!()
        }
    }

    pub fn draw_texture(&mut self, texture: RenderTexture) {
        if let Some(render_context) = &mut self.render_context {
            self.render2d_pipeline
                .draw_texture(render_context, &self.queue, texture);
        } else {
            panic!()
        }
    }

    pub fn draw_text(&mut self, _text: RenderText) {
        // _text.font.draw_vertices(
        //     _text.text,
        //     _text.position,
        //     _text.size,
        //     |texture, vertices, texture_coords| {
        //         self.draw_vertices(RenderVertices {
        //             texture: Some(texture),
        //             vertices: &vertices,
        //             texture_coords,
        //             color: _text.color,
        //         })
        //     },
        // )
    }

    pub fn draw_vertices(&mut self, _vertices: RenderVertices) {
        if let Some(render_context) = &mut self.render_context {
            self.render2d_pipeline.draw_vertices(
                render_context,
                &self.queue,
                _vertices.vertices,
                _vertices.color,
                _vertices.texture_coords,
                _vertices.texture,
            )
        } else {
            panic!()
        }
    }
}
