pub use crate::render::texture::SubTexture;
use crate::{
    core::asset_manager::TextureId,
    render::vertex::{CENTER_QUAD, TOP_LEFT_QUAD},
};

pub struct Sprite {
    pub texture_id: Option<TextureId>,
    pub sub_texture: Option<SubTexture>,
    pub color: Option<glam::Vec4>,
    pub visible: bool,
    pub center_origin: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub vertices: [glam::Vec3; 4],
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            texture_id: None,
            color: None,
            sub_texture: None,
            visible: true,
            center_origin: false,
            flip_x: false,
            flip_y: false,
            vertices: [glam::Vec3::ZERO; 4],
        }
    }
}

impl Sprite {
    pub(crate) fn update_vertices(
        &mut self,
        position: glam::Vec2,
        rotate: f32,
        scale: glam::Vec2,
        texture_size: glam::Vec2,
    ) {
        let quad = if self.center_origin {
            &CENTER_QUAD
        } else {
            &TOP_LEFT_QUAD
        };

        let render_rect_size = if let Some(sub_texture) = &self.sub_texture {
            sub_texture.region.size().into()
        } else {
            texture_size
        };

        let transform = if rotate == 0.0 {
            glam::Mat4::from_translation(position.extend(0.0))
                * glam::Mat4::from_scale(render_rect_size.extend(0.0) * scale.extend(0.0))
        } else {
            glam::Mat4::from_scale_rotation_translation(
                render_rect_size.extend(0.0) * scale.extend(0.0),
                glam::Quat::from_rotation_z(rotate),
                position.extend(0.0),
            )
        };

        self.vertices[0] = (transform * quad[0]).truncate();
        self.vertices[1] = (transform * quad[1]).truncate();
        self.vertices[2] = (transform * quad[2]).truncate();
        self.vertices[3] = (transform * quad[3]).truncate();
    }

    pub(crate) fn get_vertices(&self) -> &[glam::Vec3; 4] {
        &self.vertices
    }
}
