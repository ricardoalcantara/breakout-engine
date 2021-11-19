use bytemuck::{Pod, Zeroable};
use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

pub struct Sprite {
    x: f32,
    y: f32,
    uniform: SpriteUniform,
    pub sprite_buffer: wgpu::Buffer,
    pub sprite_bind_group: wgpu::BindGroup,
    pub sprite_bind_group_layout: wgpu::BindGroupLayout,
}

impl Sprite {
    pub fn new(x: f32, y: f32, device: &wgpu::Device) -> Self {
        let sprite_uniform = SpriteUniform::new();
        let sprite_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Buffer"),
            contents: bytemuck::cast_slice(&[sprite_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sprite_bind_group_layout =
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
                label: Some("sprite_bind_group_layout"),
            });

        let sprite_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &sprite_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: sprite_buffer.as_entire_binding(),
            }],
            label: Some("sprite_bind_group"),
        });

        Self {
            x,
            y,
            uniform: SpriteUniform::new(),
            sprite_buffer,
            sprite_bind_group,
            sprite_bind_group_layout,
        }
    }

    pub fn build_model_matrix(&self) -> cgmath::Matrix4<f32> {
        let model: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let model =
            model * cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.x, self.y, 0.0));
        let model = model * cgmath::Matrix4::from_scale(0.5);

        return model.into();
    }

    pub fn get_uniform(&mut self) -> SpriteUniform {
        let mut uniform = self.uniform;
        uniform.update_model(self);
        uniform
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteUniform {
    pub model: [[f32; 4]; 4],
}

impl SpriteUniform {
    fn new() -> Self {
        Self {
            model: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_model(&mut self, sprite: &Sprite) {
        self.model = sprite.build_model_matrix().into();
    }
}
