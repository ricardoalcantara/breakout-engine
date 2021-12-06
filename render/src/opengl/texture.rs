use image::GenericImageView;
use std::ffi::c_void;

pub struct Texture {
    // holds the ID of the texture object, used for all texture operations to reference to this particlar texture
    id: u32,
    // texture image dimensions
    width: u32,
    height: u32, // width and height of loaded image in pixels
    // texture Format
    pub internal_format: u32, // format of texture object
    pub image_format: u32,    // format of loaded image
    // texture configuration
    wrap_s: u32,     // wrapping mode on S axis
    wrap_t: u32,     // wrapping mode on T axis
    filter_min: u32, // filtering mode if texture pixels < screen pixels
    filter_max: u32, // filtering mode if texture pixels > screen pixels
}

impl Texture {
    pub fn new() -> Self {
        let id = unsafe {
            let mut id: u32 = 0;
            gl::GenTextures(1, &mut id);
            id
        };

        Self {
            id,
            width: 0,
            height: 0,
            internal_format: gl::RGB,
            image_format: gl::RGB,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
        }
    }

    pub fn generate(&mut self, file: &str) {
        let diffuse_image = image::open(file).unwrap();
        let dimensions = diffuse_image.dimensions();

        self.width = dimensions.0;
        self.height = dimensions.1;

        let diffuse_rgba = diffuse_image.as_rgba8().unwrap();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            let img_ptr: *const c_void = diffuse_rgba.as_ptr() as *const c_void;
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.internal_format as i32,
                self.width as i32,
                self.height as i32,
                0,
                self.image_format,
                gl::UNSIGNED_BYTE,
                img_ptr,
            );
            // set Texture wrap and filter modes
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                self.filter_min as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                self.filter_max as i32,
            );
            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        };
    }
}
