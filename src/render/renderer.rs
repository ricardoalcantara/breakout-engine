use super::{
    render2d_pipeline::Render2DPineline, RenderQuad, RenderText, RenderTexture, RenderVertices,
};
use log::info;
use std::rc::Rc;
use winit::window::Window;

pub struct RenderContext {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    size: glam::UVec2,
    render2d_pipeline: Render2DPineline,
    clear_color: wgpu::Color,
    display_size: Option<glam::UVec2>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let size = {
            let tmp = window.inner_size();
            glam::uvec2(tmp.width, tmp.height)
        };

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
            width: size.x,
            height: size.y,
            // TODO VSYNC
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let device = Rc::new(device);
        let queue = Rc::new(queue);

        //TODO max_texture_xyz
        let render2d_pipeline = Render2DPineline::new(size.x, size.y, 2, &device, &queue, &config);

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
            display_size: None,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn window_size(&self) -> glam::UVec2 {
        self.size
    }

    pub fn display_size(&self) -> glam::UVec2 {
        self.display_size.unwrap_or(self.size)
    }

    pub fn set_display_size(&mut self, display_size: glam::UVec2) {
        self.display_size = Some(display_size);
    }

    pub fn reconfigure(&mut self) {
        if let Some(display_size) = self.display_size {
            self.resize(display_size);
        } else {
            self.resize(self.size);
        }
    }

    pub fn resize(&mut self, new_size: glam::UVec2) {
        if new_size.x > 0 && new_size.y > 0 {
            self.size = new_size;
            self.config.width = new_size.x;
            self.config.height = new_size.y;
            self.surface.configure(&self.device, &self.config);
            if self.display_size.is_none() {
                self.render2d_pipeline.resize(new_size.x, new_size.y);
            }
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
        if let Some(camera) = camera {
            self.render2d_pipeline.set_camera(camera);
        } else {
            self.render2d_pipeline.default_camera();
        }
        self.render2d_pipeline.begin_batch();
    }

    pub fn end_draw(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_context = RenderContext {
            output,
            view,
            encoder,
        };
        self.render2d_pipeline.draw(&mut render_context);

        self.queue
            .submit(std::iter::once(render_context.encoder.finish()));
        render_context.output.present();
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        self.render2d_pipeline.draw_quad(quad);
    }

    pub fn draw_texture(&mut self, texture: RenderTexture) {
        self.render2d_pipeline.draw_texture(texture);
    }

    pub fn draw_text(&mut self, _text: RenderText) {
        _text.font.draw_vertices(
            _text.text,
            _text.position,
            _text.size,
            |texture, vertices, texture_coords| {
                self.draw_vertices(RenderVertices {
                    texture: Some(texture.clone()),
                    vertices,
                    texture_coords: texture_coords.clone(),
                    color: _text.color,
                })
            },
        )
    }

    pub fn draw_vertices(&mut self, vertices: RenderVertices) {
        self.render2d_pipeline.draw_vertices(vertices);
    }
}
