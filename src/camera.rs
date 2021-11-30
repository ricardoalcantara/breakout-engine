#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0]
);

pub struct Camera {
    matrix: glam::Mat4,
    cached_matrix: glam::Mat4,
    position: glam::Vec2,
    scale: glam::Vec2,
    size: glam::Vec2,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let position = glam::Vec2::ONE;
        let scale = glam::Vec2::ONE;
        let size = glam::Vec2::new(width, height);

        let scale_matrix = glam::Mat4::from_scale(glam::Vec3::new(scale.x, scale.y, 1.0));
        let position_matrix =
            glam::Mat4::from_translation(glam::Vec3::new(position.x, position.y, 1.0));

        // let matrix = glam::Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, -1.0, 1.0);
        let matrix = glam::Mat4::orthographic_rh_gl(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            -1.0,
            1.0,
        );
        let cached_matrix = matrix * scale_matrix; // * position_matrix;

        Self {
            matrix,
            cached_matrix,
            position,
            scale,
            size,
        }
    }

    fn cache_matrix(&mut self) {
        let scale_matrix = glam::Mat4::from_scale(glam::Vec3::new(self.scale.x, self.scale.y, 1.0));
        let position_matrix =
            glam::Mat4::from_translation(glam::Vec3::new(self.position.x, self.position.y, 1.0));

        self.cached_matrix = self.matrix * scale_matrix; // * position_matrix;
    }

    pub fn get_matrix(&self) -> glam::Mat4 {
        self.cached_matrix
    }

    pub fn get_position(&mut self) -> glam::Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: glam::Vec2) {
        self.position = position;
        self.cache_matrix();
    }

    pub fn get_scale(&mut self) -> glam::Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: glam::Vec2) {
        self.scale = scale;
        self.cache_matrix();
    }

    pub fn get_size(&mut self) -> glam::Vec2 {
        self.size
    }

    pub fn set_size(&mut self, size: glam::Vec2) {
        self.size = size;
        self.cache_matrix();
    }
}
