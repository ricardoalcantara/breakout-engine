use bytemuck::{Pod, Zeroable};
use cgmath::SquareMatrix;

pub struct Sprite {
    x: f32,
    y: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn build_model_matrix(&self) -> cgmath::Matrix4<f32> {
        let model: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let model = model * cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.x, self.y, 0.0));
        // let model = model * cgmath::Matrix4::from_scale(200.0);


        return model.into()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteUniform {
    pub model: [[f32; 4]; 4],
}

impl SpriteUniform {
    pub fn new() -> Self {
        Self {
            model: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_model(&mut self, sprite: &Sprite) {
        self.model = sprite.build_model_matrix().into();
    }
}
