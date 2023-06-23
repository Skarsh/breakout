use crate::{
    game_object::GameObject,
    graphics::{sprite_renderer::SpriteRenderer, texture::Texture2D},
};
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Ball {
    object: GameObject,
    // ball state
    radius: f32,
    stuck: bool,
}

impl Ball {
    pub fn new(object: GameObject, radius: f32, stuck: bool) -> Self {
        Self {
            object,
            radius,
            stuck,
        }
    }

    pub fn move_ball(&mut self, dt: f32, window_width: u32) -> glm::Vec2 {
        // if not stuck to player board
        if !self.stuck {
            self.object.position += self.object.velocity * dt;
            if self.object.position.x <= 0.0 {
                self.object.velocity.x = -self.object.velocity.x;
                self.object.position.x = 0.0;
            } else if self.object.position.x + self.object.size.x >= window_width as f32 {
                self.object.velocity.x = -self.object.velocity.x;
                self.object.position.x = window_width as f32 - self.object.size.x;
            }
            if self.object.position.y <= 0.0 {
                self.object.velocity.y = -self.object.velocity.y;
                self.object.position.y = 0.0;
            }
        }

        self.object.position
    }

    pub fn draw(&self, renderer: &mut SpriteRenderer, sprite: &Texture2D) {
        self.object.draw(renderer, sprite);
    }

    pub fn reset(&mut self, position: glm::Vec2, velocity: glm::Vec2) {
        self.object.position = position;
        self.object.velocity = velocity;
        self.stuck = true;
    }
}
