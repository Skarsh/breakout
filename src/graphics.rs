use std::{ffi::CString, path::Path};

use nalgebra_glm as glm;

use crate::{
    shader::Shader, shader_manager::ShaderManager, sprite_renderer::SpriteRenderer,
    texture_manager::TextureManager,
};

#[derive(Debug)]
pub struct Graphics {
    width: u32,
    height: u32,
    shader_manager: ShaderManager,
    texture_manager: TextureManager,
    sprite_renderer: SpriteRenderer,
}

impl Graphics {
    pub fn new(
        width: u32,
        height: u32,
        mut shader_manager: ShaderManager,
        mut texture_manager: TextureManager,
    ) -> Self {
        let shader = shader_manager.load_shader(
            Path::new("shaders/sprite.vs"),
            Path::new("shaders/sprite.frag"),
            None,
            "sprite".to_string(),
        );

        let projection = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        shader_manager
            .get_shader("sprite")
            .use_program()
            .set_int(&CString::new("image").unwrap(), 0);

        shader_manager
            .get_shader("sprite")
            .set_mat4(&CString::new("projection").unwrap(), &projection);

        texture_manager.load_texture(
            Path::new("resources/textures/awesomeface.png"),
            true,
            "face",
        );

        Self {
            width,
            height,
            shader_manager,
            texture_manager,
            sprite_renderer: SpriteRenderer::new(shader),
        }
    }

    pub fn render(&mut self) {
        self.sprite_renderer.draw_sprite(
            self.texture_manager.get_texture("face"),
            glm::vec2(200.0, 200.0),
            glm::vec2(300.0, 400.0),
            45.0,
            glm::vec3(0.0, 1.0, 0.0),
        );
    }
}
