use std::path::Path;

use nalgebra_glm as glm;

use crate::{
    shader_manager::ShaderManager, sprite_renderer::SpriteRenderer, texture_manager::TextureManager,
};

#[derive(Debug)]
pub struct Graphics {
    width: u32,
    height: u32,
    shader_manager: ShaderManager,
    texture_manager: TextureManager,
    sprite_renderer: Option<SpriteRenderer>,
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            shader_manager: ShaderManager::new(),
            texture_manager: TextureManager::new(),
            sprite_renderer: None,
        }
    }

    pub fn init(&mut self) {
        let shader = self.shader_manager.load_shader(
            Path::new("shaders/sprite.vs"),
            Path::new("shaders/sprite.frag"),
            None,
            "sprite".to_string(),
        );

        let projection = glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);

        self.shader_manager
            .get_shader("sprite")
            .use_program()
            .set_int("image", 0);
        self.shader_manager
            .get_shader("sprite")
            .set_mat4("projection", &projection);

        let renderer = SpriteRenderer::new();
        self.sprite_renderer = Some(renderer);
        self.texture_manager.load_texture(
            Path::new("resources/textures/awesomeface.png"),
            true,
            "face",
        );
    }

    pub fn render(&mut self) {
        self.sprite_renderer.as_mut().unwrap().draw_sprite(
            self.shader_manager.get_shader("sprite"),
            self.texture_manager.get_texture("face"),
            glm::vec2(200.0, 200.0),
            glm::vec2(300.0, 400.0),
            45.0,
            glm::vec3(0.0, 1.0, 0.0),
        );
    }
}
