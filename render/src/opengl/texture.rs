use image::GenericImageView;
use std::ffi::c_void;

use crate::texture::{Texture, TextureType};

pub struct OpenGLTexture {
    // holds the ID of the texture object, used for all texture operations to reference to this particlar texture
    id: u32,
    // texture Format
    pub internal_format: u32, // format of texture object
    pub image_format: u32,    // format of loaded image
    // texture configuration
    wrap_s: u32,     // wrapping mode on S axis
    wrap_t: u32,     // wrapping mode on T axis
    filter_min: u32, // filtering mode if texture pixels < screen pixels
    filter_max: u32, // filtering mode if texture pixels > screen pixels
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
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
        }
    }

    pub fn generate_texture(diffuse_image: image::DynamicImage) -> Texture {
        let dimensions = diffuse_image.dimensions();

        let width = dimensions.0;
        let height = dimensions.1;

        let mut opengl_texture = OpenGLTexture::new();

        let data_ptr = match diffuse_image.color() {
            image::ColorType::Rgba8 => {
                opengl_texture.internal_format = gl::RGBA;
                opengl_texture.image_format = gl::RGBA;
                let data = diffuse_image.as_rgba8().unwrap();
                let img_ptr: *const c_void = data.as_ptr() as *const c_void;
                img_ptr
            }
            image::ColorType::Rgb8 => {
                opengl_texture.internal_format = gl::RGB;
                opengl_texture.image_format = gl::RGB;
                let data = diffuse_image.as_rgb8().unwrap();
                let img_ptr: *const c_void = data.as_ptr() as *const c_void;
                img_ptr
            }
            _ => panic!("ColorType not supportes"),
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, opengl_texture.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                opengl_texture.internal_format as i32,
                width as i32,
                height as i32,
                0,
                opengl_texture.image_format,
                gl::UNSIGNED_BYTE,
                data_ptr,
            );
            // set Texture wrap and filter modes
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                opengl_texture.wrap_s as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                opengl_texture.wrap_t as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                opengl_texture.filter_min as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                opengl_texture.filter_max as i32,
            );
            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        };

        Texture {
            width,
            height,
            texture_type: TextureType::OpenGL(opengl_texture),
        }
    }

    pub fn generate(&mut self, diffuse_image: image::DynamicImage) {}

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        };
    }
}
