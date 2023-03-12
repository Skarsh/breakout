use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, path::Path};

use crate::{shader::Shader, texture::Texture};

pub struct ResourceManager<'a> {
    shaders: HashMap<&'a str, Shader>,
    textures: HashMap<&'a str, Texture>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_shader(
        &'a mut self,
        v_shader_file: &Path,
        f_shader_file: &Path,
        g_shader_file: Option<&Path>,
        name: &'a str,
    ) -> &'a Shader {
        self.shaders.insert(
            name,
            load_shader_from_file(v_shader_file, f_shader_file, g_shader_file),
        );
        // TODO: Deal with unwrap
        self.shaders.get("name").unwrap()
    }

    pub fn get_shader(&'a self, name: &'a str) -> &'a Shader {
        // TODO: Deal with unwrap
        self.shaders.get(name).unwrap()
    }

    pub fn load_texture(&'a mut self, file: &Path, alpha: bool, name: &'a str) -> &'a Texture {
        self.textures
            .insert(name, load_texture_from_file(file, alpha));
        self.textures.get(name).unwrap()
    }

    pub fn get_texture(&'a self, name: &'a str) -> &'a Texture {
        self.textures.get(name).unwrap()
    }

    pub fn clear(&self) {
        for shader in self.shaders.iter() {
            unsafe {
                gl::DeleteProgram(shader.1.id);
            }
        }

        for texture in self.textures.iter() {
            unsafe {
                gl::DeleteTextures(1, &texture.1.id);
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

    let shader = Shader::new(v_shader_code, f_shader_code, None);

    shader
}

fn load_texture_from_file(file: &Path, alpha: bool) -> Texture {
    let mut texture = Texture::new();

    if alpha {
        texture.internal_format = gl::RGBA;
        texture.image_format = gl::RGBA;
    }

    let image = image::open(file).unwrap();
    texture.generate(image.width(), image.height(), image.as_bytes());

    texture
}
