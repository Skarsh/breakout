use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;
use std::str;

use gl::types::*;

use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    //pub fn new(vertex_path: &Path, fragment_path: &Path, geometry_path: Option<&Path>) -> Self {
    pub fn new(
        v_shader_code: CString,
        f_shader_code: CString,
        geometry_path: Option<CString>,
    ) -> Self {
        let mut shader = Shader { id: 0 };

        //// 1. retrieve the vertex/fragment source code from filesystem
        //let mut v_shader_file = File::open(vertex_path)
        //    .unwrap_or_else(|_| panic!("failed to open file {}", vertex_path.display()));

        //let mut f_shader_file = File::open(fragment_path)
        //    .unwrap_or_else(|_| panic!("failed to open file {}", fragment_path.display()));

        //let mut vertex_code = String::new();
        //let mut fragment_code = String::new();

        //v_shader_file
        //    .read_to_string(&mut vertex_code)
        //    .expect("Failed to read vertex shader");
        //f_shader_file
        //    .read_to_string(&mut fragment_code)
        //    .expect("Failed to read fragment shader");

        //let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        //let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    /// actiavate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_vec2(&self, name: &CStr, value: &Vec2) {
        gl::Uniform2fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    pub unsafe fn set_vec2_xyz(&self, name: &CStr, x: f32, y: f32) {
        gl::Uniform2f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y);
    }

    pub unsafe fn set_vec3(&self, name: &CStr, value: &Vec3) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    pub unsafe fn set_vec3_xyz(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    pub unsafe fn set_vec4(&self, name: &CStr, value: &Vec4) {
        gl::Uniform4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    pub unsafe fn set_vec4_xyz(&self, name: &CStr, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z, w);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Mat4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, r#type: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);
        if r#type != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n\
                          -- -------------------------------------------------- -- ",
                    r#type,
                    str::from_utf8(&info_log).unwrap()
                )
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n\
                          -- -------------------------------------------------- -- ",
                    r#type,
                    str::from_utf8(&info_log).unwrap()
                )
            }
        }
    }
}
