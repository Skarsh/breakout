use std::{collections::HashMap, ffi::CString, fs::File, io::Read, path::Path};

use crate::shader::Shader;

#[derive(Debug)]
pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        v_shader_file: &Path,
        f_shader_file: &Path,
        g_shader_file: Option<&Path>,
        name: String,
    ) -> &Shader {
        self.shaders.insert(
            name.clone(),
            load_shader_from_file(v_shader_file, f_shader_file, g_shader_file),
        );
        // TODO: Deal with unwrap
        self.shaders.get(&name).unwrap()
    }

    pub fn get_shader(&self, name: &str) -> &Shader {
        // TODO: Deal with unwrap
        self.shaders.get(name).unwrap()
    }

    pub fn clear(&self) {
        for shader in self.shaders.iter() {
            unsafe {
                gl::DeleteProgram(shader.1.id);
            }
        }
    }
}

fn load_shader_from_file(
    v_shader_file: &Path,
    f_shader_file: &Path,
    g_shader_file: Option<&Path>,
) -> Shader {
    let mut v_shader_file = File::open(v_shader_file)
        .unwrap_or_else(|_| panic!("failed to open file {}", v_shader_file.display()));
    let mut f_shader_file = File::open(f_shader_file)
        .unwrap_or_else(|_| panic!("failed to open file {}", f_shader_file.display()));

    let mut vertex_code = String::new();
    let mut fragment_code = String::new();

    v_shader_file
        .read_to_string(&mut vertex_code)
        .expect("failed to read vertex shader");
    f_shader_file
        .read_to_string(&mut fragment_code)
        .expect("failed to read fragment shader");

    let v_shader_code =
        CString::new(vertex_code.as_bytes()).expect("failed to read vertex code into CString");
    let f_shader_code =
        CString::new(fragment_code.as_bytes()).expect("failed to read fragment code into CString");

    Shader::new(v_shader_code, f_shader_code, None)
}
