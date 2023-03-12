use std::{collections::HashMap, io::Read, path::Path};

use crate::texture::Texture;

#[derive(Debug)]
pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, file: &Path, alpha: bool, name: &str) -> &Texture {
        self.textures
            .insert(name.to_string(), load_texture_from_file(file, alpha));
        self.textures.get(name).unwrap()
    }

    pub fn get_texture(&self, name: &str) -> &Texture {
        self.textures.get(name).unwrap()
    }

    pub fn clear(&self) {
        for texture in self.textures.iter() {
            unsafe {
                gl::DeleteTextures(1, &texture.1.id);
            }
        }
    }
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
