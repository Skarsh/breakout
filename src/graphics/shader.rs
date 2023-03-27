use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use gl::types::*;

use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

#[derive(Debug)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    //pub fn new(vertex_path: &Path, fragment_path: &Path, geometry_path: Option<&Path>) -> Self {
    pub fn new(
        v_shader_code: CString,
        f_shader_code: CString,
        geometry_code: Option<CString>,
    ) -> Self {
        let mut shader = Shader { id: 0 };

        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            check_compile_errors(fragment, "FRAGMENT");

            // geometry shader
            let mut geometry: u32 = 0;
            if let Some(source) = geometry_code.as_ref() {
                geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
                gl::ShaderSource(geometry, 1, &source.as_ptr(), ptr::null());
                gl::CompileShader(geometry);
                check_compile_errors(geometry, "GEOMETRY");
            }

            // shader program
            shader.id = gl::CreateProgram();
            gl::AttachShader(shader.id, vertex);
            gl::AttachShader(shader.id, fragment);
            if let Some(source) = geometry_code.as_ref() {
                gl::AttachShader(shader.id, geometry);
            }
            gl::LinkProgram(shader.id);
            check_compile_errors(shader.id, "PROGRAM");

            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            if let Some(source) = geometry_code {
                gl::DeleteShader(geometry)
            }
        }

        shader
    }

    /// activate the shader
    pub fn use_program(&self) -> &Shader {
        unsafe {
            gl::UseProgram(self.id);
        }
        self
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(
                    self.id,
                    CStr::from_bytes_with_nul_unchecked(name.as_bytes()).as_ptr(),
                ),
                value,
            );
        }
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

    pub fn set_vec3(&self, name: &str, value: &Vec3) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(
                    self.id,
                    CStr::from_bytes_with_nul_unchecked(name.as_bytes()).as_ptr(),
                ),
                1,
                value.as_ptr(),
            );
        }
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

    pub fn set_mat4(&self, name: &str, mat: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                // TODO: No idea if this casting is safe
                gl::GetUniformLocation(
                    self.id,
                    CStr::from_bytes_with_nul_unchecked(name.as_bytes()).as_ptr(),
                ),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }
}

unsafe fn check_compile_errors(shader: u32, r#type: &str) {
    let mut success = gl::FALSE as GLint;
    let mut info_log: Vec<u8> = Vec::with_capacity(1024);
    for val in &mut info_log {
        *val = 0;
    }

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
