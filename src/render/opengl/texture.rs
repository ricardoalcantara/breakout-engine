use crate::{
    error::BreakoutResult,
    render::texture::{Texture, TextureType},
};

use super::check_gl_ok;
use image::GenericImageView;
use std::ffi::c_void;

pub struct OpenGLTexture {
    // holds the ID of the texture object, used for all texture operations to reference to this particlar texture
    pub(crate) id: u32,
    // texture Format
    pub internal_format: u32, // format of texture object
    pub image_format: u32,    // format of loaded image
    // texture configuration
    wrap_s: u32,     // wrapping mode on S axis
    wrap_t: u32,     // wrapping mode on T axis
    filter_min: u32, // filtering mode if texture pixels < screen pixels
    filter_max: u32, // filtering mode if texture pixels > screen pixels
}

impl Drop for OpenGLTexture {
    fn drop(&mut self) {
        log::debug!("DeleteTextures {:}", &self.id);
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl OpenGLTexture {
    fn new() -> Self {
        let id = unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            id
        };

        Self {
            id,
            internal_format: gl::RGBA,
            image_format: gl::RGBA,
            wrap_s: gl::CLAMP_TO_EDGE,
            wrap_t: gl::CLAMP_TO_EDGE,
            filter_min: gl::NEAREST,
            filter_max: gl::NEAREST,
        }
    }

    pub fn generate_texture(diffuse_image: image::DynamicImage) -> BreakoutResult<Texture> {
        let dimensions = diffuse_image.dimensions();

        let width = dimensions.0;
        let height = dimensions.1;

        let data = diffuse_image.to_bytes();
        // let img_ptr = data.as_ptr() as *const c_void;

        let mut opengl_texture = OpenGLTexture::new();

        match diffuse_image.color() {
            image::ColorType::Rgba8 => {
                opengl_texture.internal_format = gl::RGBA;
                opengl_texture.image_format = gl::RGBA;
            }
            image::ColorType::Rgb8 => {
                opengl_texture.internal_format = gl::RGB;
                opengl_texture.image_format = gl::RGB;
            }
            _ => panic!("ColorType not supportes"),
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, opengl_texture.id);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, opengl_texture.wrap_s as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, opengl_texture.wrap_t as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, opengl_texture.filter_min as i32);
            #[rustfmt::skip]
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, opengl_texture.filter_max as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                opengl_texture.internal_format as i32,
                width as i32,
                height as i32,
                0,
                opengl_texture.image_format,
                gl::UNSIGNED_BYTE,
                std::mem::transmute::<*const u8, *const c_void>(data.as_ptr()),
                // img_ptr,
            );
            check_gl_ok()?;
            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        };

        Ok(Texture {
            width,
            height,
            texture_type: TextureType::OpenGL(opengl_texture),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        };
    }
}
