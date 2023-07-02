use std::{collections::HashMap, ffi::c_void};

use freetype as ft;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use nalgebra_glm as glm;

use super::shader::Shader;

struct Character {
    texture_id: u32,
    size: glm::IVec2,
    bearing: glm::IVec2,
    advance: u32,
}

pub struct TextRenderer {
    characters: HashMap<u8, Character>,
    text_shader: Shader,
    vao: u32,
    vbo: u32,
}

impl TextRenderer {
    pub fn new(width: u32, height: u32, text_shader: Shader) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        text_shader.use_program();
        // TODO: No clue whether the near and far values here makes sense
        text_shader.set_mat4(
            "projection\0",
            &glm::ortho(0.0, width as f32, height as f32, 0.0, 0.0, 1.0),
        );
        text_shader.set_int("text\0", 0);

        // configure VAO/VBO for texture quads
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * 6 * 4) as GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            characters: HashMap::new(),
            text_shader,
            vao,
            vbo,
        }
    }

    pub fn load(&mut self, font: String, font_size: u32) {
        self.characters.clear();
        let library =
            ft::Library::init().expect("ERROR::FREETYPE: Could not init FreeType Library");
        let face = library
            .new_face(font, 0)
            .expect("ERROR::FREETYPE: Failed to load font");
        face.set_pixel_sizes(0, font_size).unwrap();

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            for c in 0..128 {
                // TODO: Should we not panic here? Just continue
                face.load_char(c, ft::face::LoadFlag::RENDER).unwrap();

                // generate texture
                let mut texture = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                //gl::TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels)
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    face.glyph().bitmap().width(),
                    face.glyph().bitmap().rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    face.glyph().bitmap().buffer().as_ptr() as *const c_void,
                );

                // set texture options
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                // now store character for later use
                let character = Character {
                    texture_id: texture,
                    size: glm::IVec2::new(
                        face.glyph().bitmap().width(),
                        face.glyph().bitmap().rows(),
                    ),
                    bearing: glm::IVec2::new(face.glyph().bitmap_left(), face.glyph().bitmap_top()),
                    advance: face.glyph().advance().x as u32,
                };
                self.characters.insert(c as u8, character);
            }
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn render_text(&mut self, text: &str, mut x: f32, y: f32, scale: f32, color: glm::Vec3) {
        // activate corresponding render state
        self.text_shader.use_program();
        self.text_shader.set_vec3("textColor\0", &color);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);

            // iterate through all characters
            for c in text.as_bytes() {
                let ch = &self.characters.get(c).unwrap();
                let x_pos = x + ch.bearing.x as f32 * scale;
                let y_pos = y
                    + (self.characters.get(&b'H').unwrap().bearing.y - ch.bearing.y) as f32 * scale;

                let w = ch.size.x as f32 * scale;
                let h = ch.size.y as f32 * scale;

                // update VBO for each character
                #[rustfmt::skip]
                let vertices: [[f32;4]; 6] = [
                    [x_pos,     y_pos + h, 0.0, 1.0],
                    [x_pos + w, y_pos,     1.0, 0.0],
                    [x_pos,     y_pos,     0.0, 0.0],

                    [x_pos,     y_pos + h, 0.0, 1.0],
                    [x_pos + w, y_pos + h, 1.0, 1.0],
                    [x_pos + w, y_pos,     1.0, 0.0],
                ];

                // render glyph texture over quad
                gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
                // update content of VBO memory
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    std::mem::size_of_val(&vertices) as GLsizeiptr,
                    vertices.as_ptr() as *const c_void,
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);

                // update quad
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                // now advance cursors for next glyph
                x += (ch.advance >> 6) as f32 * scale; // bitshift by 6 to get value in pixels
            }
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
