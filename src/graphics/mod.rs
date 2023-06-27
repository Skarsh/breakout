#![allow(dead_code)]
pub mod shader;
pub mod shader_manager;
pub mod sprite_renderer;
pub mod texture;
pub mod texture_manager;

use std::path::Path;

use nalgebra_glm as glm;

use shader_manager::ShaderManager;
use sprite_renderer::SpriteRenderer;
use texture_manager::TextureManager;

#[derive(Debug)]
pub struct Graphics {
    pub width: u32,
    pub height: u32,
    pub shader_manager: ShaderManager,
    pub texture_manager: TextureManager,
    pub sprite_renderer: SpriteRenderer,
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
            .set_int("image\0", 0);

        shader_manager
            .get_shader("sprite")
            .set_mat4("projection\0", &projection);

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
            self.texture_manager.get_texture("background"),
            glm::vec2(0.0, 0.0),
            glm::vec2(self.width as f32, self.height as f32),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
        );
    }

    pub fn clear(&mut self) {
        self.texture_manager.clear();
        self.shader_manager.clear();
    }
}
