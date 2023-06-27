#![allow(dead_code)]
use nalgebra_glm as glm;

use crate::{graphics::sprite_renderer::SpriteRenderer, graphics::texture::Texture2D};

#[derive(Debug, Clone)]
pub struct GameObject {
    // object state
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub velocity: glm::Vec2,
    pub color: glm::Vec3,
    pub rotation: f32,
    pub is_solid: bool,
    pub destroyed: bool,
    // render state
    pub sprite_id: String,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            position: glm::vec2(0.0, 0.0),
            size: glm::vec2(1.0, 1.0),
            velocity: glm::vec2(0.0, 0.0),
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite_id: String::new(),
        }
    }
}

impl GameObject {
    pub fn new(
        position: glm::Vec2,
        size: glm::Vec2,
        color: glm::Vec3,
        velocity: glm::Vec2,
        sprite_id: String,
    ) -> Self {
        Self {
            position,
            size,
            velocity,
            color,
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite_id,
        }
    }

    pub fn draw(&self, renderer: &mut SpriteRenderer, sprite: &Texture2D) {
        renderer.draw_sprite(sprite, self.position, self.size, self.rotation, self.color);
    }

    pub fn sprite_id(&self) -> &String {
        &self.sprite_id
    }
}
