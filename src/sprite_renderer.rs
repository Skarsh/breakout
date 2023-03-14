use std::ffi::{c_void, CString};
use std::rc::Rc;
use std::{mem, ptr};

use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Vec2, Vec3};

use crate::{shader::Shader, texture::Texture};

#[derive(Debug)]
pub struct SpriteRenderer {
    quad_vao: GLuint,
    shader: Rc<Shader>,
}

impl SpriteRenderer {
    pub fn new(shader: Rc<Shader>) -> Self {
        let mut quad_vao: u32 = 0;
        init_render_data(&mut quad_vao);
        Self { quad_vao, shader }
    }

    pub fn draw_sprite(
        &mut self,
        texture: &Texture,
        position: Vec2,
        size: Vec2,
        rotate: f32,
        color: Vec3,
    ) {
        // prepare transformations
        self.shader.use_program();
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

        self.shader
            .set_mat4(&CString::new("model").unwrap(), &model);

        self.shader
            .set_vec3(&CString::new("spriteColor").unwrap(), &color);

        // TODO: Can we abstract the unsafeness away to make no unsafe code in draw call?
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}

fn init_render_data(quad_vao: &mut u32) {
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
        gl::GenVertexArrays(1, quad_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(*quad_vao);

        let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}
