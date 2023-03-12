use std::ffi::{c_void, CStr};
use std::mem::size_of;
use std::ptr;

use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Vec2, Vec3};

use crate::{shader::Shader, texture::Texture};

#[derive(Debug)]
pub struct SpriteRenderer {
    quad_vao: GLuint,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        Self { quad_vao: 0 }
    }

    pub fn draw_sprite(
        &mut self,
        shader: &Shader,
        texture: &Texture,
        position: Vec2,
        size: Vec2,
        rotate: f32,
        color: Vec3,
    ) {
        // prepare transformations
        shader.use_program();
        let mut model = Mat4::identity();

        // first translate (transformations are: scale happens first,
        // then rotation, and then final translation; reversed order)
        model = glm::translate(&model, &glm::vec3(position.x, position.y, 0.0));

        // move origin of rotation to center of quad
        model = glm::translate(&model, &glm::vec3(0.5 * size.x, 0.5 * size.y, 0.0));

        // then rotate
        model = glm::rotate(&model, rotate.to_radians(), &glm::vec3(0.0, 0.0, 1.0));

        // move origin back
        model = glm::translate(&model, &glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        // scale
        model = glm::scale(&model, &glm::vec3(size.x, size.y, 1.0));

        shader.set_mat4("model", &model);

        shader.set_vec3("spriteColor", &color);

        // TODO: Can we abstract the unsafety away to make no unsafe code in draw call?
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    fn init_render_data(&mut self) {
        let mut vbo = 0;

        #[rustfmt::skip]
        let vertices: [f32; 24] = [
            // pos      // tex
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(self.quad_vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
