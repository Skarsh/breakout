use std::path::Path;

use nalgebra_glm as glm;

use crate::{
    graphics::Graphics, shader_manager::ShaderManager, sprite_renderer::SpriteRenderer,
    texture_manager::TextureManager,
};

#[derive(Debug)]
enum GameState {
    Active,
    Menu,
    Win,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    pub keys: [bool; 1024],
    pub graphics: Graphics,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        Self {
            state: GameState::Active,
            keys: [false; 1024],
            graphics,
        }
    }

    pub fn init(&mut self) {}

    pub fn process_input(&mut self, dt: f64) {}

    pub fn update(&mut self, dt: f64) {}

    pub fn render(&mut self) {
        self.graphics.render();
    }

    pub fn clear(&mut self) {
        //self.texture_manager.clear();
        //self.shader_manager.clear();
    }
}
