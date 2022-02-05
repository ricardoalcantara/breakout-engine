use std::num::NonZeroU32;

use image::DynamicImage;

pub struct Texture {
    pub id: Option<i32>,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_file(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let bytes = std::fs::read(path).unwrap();
        Texture::from_byte(&bytes, device, queue)
    }

    pub fn from_dynamic_image(
        image: DynamicImage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Texture {
        let data = image.as_rgba8().unwrap();

        use image::GenericImageView;
        let (width, height) = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture"),
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        queue.write_texture(
            texture.as_image_copy(),
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            texture_size,
        );

        Texture {
            id: None,
            texture,
            view,
            sampler,
            width,
            height,
        }
    }

    pub fn from_byte(bytes: &[u8], device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let image = image::load_from_memory(bytes).unwrap();
        Texture::from_dynamic_image(image, device, queue)
    }

    pub fn from_color(color: [u8; 4], device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let texture_size = wgpu::Extent3d::default();

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d::default(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture"),
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        queue.write_texture(
            texture.as_image_copy(),
            &color,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(4).unwrap()),
                rows_per_image: None,
            },
            texture_size,
        );

        Texture {
            id: None,
            texture,
            view,
            sampler,
            width: texture_size.width,
            height: texture_size.height,
        }
    }

    pub fn size(&self) -> glam::UVec2 {
        glam::uvec2(self.width, self.height)
    }
}
