use std::ffi::{c_void, CStr};

use gl::types::{GLsizei, GLsizeiptr};

use super::{shader::Shader, texture::Texture2D};

struct PostProcessor {
    post_processing_shader: Shader,
    texture: Texture2D,
    width: i32,
    height: i32,
    chaos: bool,
    confuse: bool,
    shake: bool,
    msfbo: u32, // MSFBO = Multisampled FBO. FBO is regular, used for blitting MS color-buffer to texture.
    fbo: u32,
    rbo: u32, // RBO is used for multisampled color buffer.
    vao: u32,
}

impl PostProcessor {
    pub fn new(shader: Shader, width: i32, height: i32) -> Self {
        let mut msfbo = 0;
        let mut fbo = 0;
        let mut rbo = 0;
        let mut texture = Texture2D::new();
        let mut vao = 0;

        let offset = 1.0 / 300.0;

        #[rustfmt::skip]
        let offsets: [[f32; 2]; 9] = [
            [ -offset,  offset ],
            [  0.0,     offset ],
            [  offset,  offset ],
            [ -offset,  0.0    ],
            [  0.0,     0.0    ],
            [  offset,  0.0    ],
            [ -offset, -offset ],
            [  0.0,    -offset ],
            [  offset, -offset ],
        ];

        #[rustfmt::skip]
        let edge_kernel: [i32; 9] = [
            -1, -1, -1,
            -1,  8, -1,
            -1, -1, -1
        ];

        #[rustfmt::skip]
        let blur_kernel: [f32; 9] = [
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
            2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
        ];

        // initialize renderbuffer/framebuffer object
        unsafe {
            gl::GenFramebuffers(1, &mut msfbo);
            gl::GenFramebuffers(1, &mut fbo);
            gl::GenFramebuffers(1, &mut rbo);

            // initialize renderbuffer storage with multisampled color buffer (don't need depth/stencil buffer)
            gl::BindFramebuffer(gl::FRAMEBUFFER, msfbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

            // allocate storage for render buffer object
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 4, gl::RGB, width, height);

            // attach MS render buffer object to framebuffer
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                eprintln!("ERROR::POSTPROCESSOR: Failed to initialize MSFBO");
            }

            // also intialize the FBO/texture to blit multisampled color-buffer to; used for shader operations(for postprocessing effects)
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            texture.generate(width, height, &[]);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.id,
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                eprintln!("ERROR::POSTPROCESSOR: Failed to initialize FBO");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // initialize render data and uniforms
            Self::init_render_data(&mut vao);

            shader.set_int("scene\0", 0);
            shader.use_program();

            gl::Uniform2fv(
                gl::GetUniformLocation(
                    shader.id,
                    CStr::from_bytes_with_nul_unchecked("offsets\0".as_bytes()).as_ptr(),
                ),
                9,
                offsets.as_ptr() as *const f32,
            );

            gl::Uniform1iv(
                gl::GetUniformLocation(
                    shader.id,
                    CStr::from_bytes_with_nul_unchecked("edge_kernel\0".as_bytes()).as_ptr(),
                ),
                9,
                edge_kernel.as_ptr(),
            );

            gl::Uniform1fv(
                gl::GetUniformLocation(
                    shader.id,
                    CStr::from_bytes_with_nul_unchecked("blur_kernel\0".as_bytes()).as_ptr(),
                ),
                9,
                blur_kernel.as_ptr(),
            );
        }

        Self {
            post_processing_shader: shader,
            texture,
            width,
            height,
            chaos: false,
            confuse: false,
            shake: false,
            msfbo,
            fbo,
            rbo,
            vao,
        }
    }

    // prepares the postprocessor's framebuffer operations before rendering the game
    pub fn begin_render(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.msfbo);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn end_render(&self) {
        // now resolve multisampled color-buffer into intermediate FBO to store to texture
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.msfbo);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo);
            gl::BlitFramebuffer(
                0,
                0,
                self.width,
                self.height,
                0,
                0,
                self.width,
                self.height,
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );
            // binds both READ and WRITE framebuffer to default framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn render(&self, time: f32) {
        // set uniforms/options
        self.post_processing_shader.use_program();
        unsafe {
            self.post_processing_shader.set_float("time\0", time);
            // TODO: Should this be set_int instead?
            self.post_processing_shader
                .set_bool("confuse\0", self.confuse);
            self.post_processing_shader.set_bool("chaos\0", self.chaos);
            self.post_processing_shader.set_bool("shake\0", self.shake);

            // render texture quad
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    fn init_render_data(vao: &mut u32) {
        let mut vbo = 0;

        #[rustfmt::skip]
        let vertices: [f32; 24] = [
            // pos        // tex
            -1.0, -1.0, 0.0, 0.0,
             1.0,  1.0, 1.0, 1.0,
            -1.0,  1.0, 0.0, 1.0,

            -1.0, -1.0, 0.0, 0.0,
             1.0, -1.0, 1.0, 0.0,
             1.0,  1.0, 1.0, 1.0
        ];

        unsafe {
            gl::GenVertexArrays(1, vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(*vao);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<f32>() as GLsizei,
                std::ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
