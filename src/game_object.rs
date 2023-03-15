use nalgebra_glm as glm;

use crate::{sprite_renderer::SpriteRenderer, texture::Texture2D};

#[derive(Debug)]
pub struct GameObject<'a> {
    // object state
    position: glm::Vec2,
    size: glm::Vec2,
    velocity: glm::Vec2,
    color: glm::Vec3,
    rotation: f32,
    pub is_solid: bool,
    pub destroyed: bool,
    // render state
    sprite: &'a Texture2D,
}

impl<'a> GameObject<'a> {
    pub fn new(
        position: glm::Vec2,
        size: glm::Vec2,
        color: glm::Vec3,
        velocity: glm::Vec2,
        sprite: &'a Texture2D,
    ) -> Self {
        Self {
            position,
            size,
            velocity,
            color,
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite,
        }
    }

    pub fn draw(&self, renderer: &mut SpriteRenderer) {
        renderer.draw_sprite(
            &self.sprite,
            self.position,
            self.size,
            self.rotation,
            self.color,
        );
    }
}
