use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // let view = cgmath::ortho(0.0, self.width, self.height, 0.0, -1.0, 1.0);
        let view = cgmath::ortho(-2.0, 2.0, -1.5, 1.5, -1.0, 1.0);
        return OPENGL_TO_WGPU_MATRIX * view;

        // let view = cgmath::Matrix4::look_at_rh(
        //     (0.0, 1.0, 2.0).into(),
        //     (0.0, 0.0, 0.0).into(),
        //     cgmath::Vector3::unit_y(),
        // );
        // let proj = cgmath::perspective(cgmath::Deg(45.0), 800.0 / 600.0, 0.1, 100.0);
        // return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            projection: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.projection = camera.build_view_projection_matrix().into();
    }
}
