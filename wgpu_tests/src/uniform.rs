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
