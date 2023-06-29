use std::ffi::c_void;

use gl::types::{GLenum, GLint, GLuint};

#[derive(Debug, Clone)]
pub struct Texture2D {
    pub id: GLuint,
    width: u32,
    height: u32,
    pub internal_format: GLint,
    pub image_format: GLenum,
    wrap_s: GLenum,
    wrap_t: GLenum,
    filter_min: GLint,
    filter_max: GLint,
}

impl Texture2D {
    pub fn new() -> Self {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }

        Self {
            id: texture,
            width: 0,
            height: 0,
            internal_format: gl::RGB as GLint,
            image_format: gl::RGB,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR as i32,
            filter_max: gl::LINEAR as i32,
        }
    }

    pub fn generate(&mut self, width: i32, height: i32, data: &[u8]) {
        self.width = width as u32;
        self.height = height as u32;

        // create texture
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.internal_format,
                width as i32,
                height as i32,
                0,
                self.image_format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_max);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
