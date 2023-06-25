use crate::{
    game_object::GameObject,
    graphics::{sprite_renderer::SpriteRenderer, texture::Texture2D},
};
use nalgebra_glm as glm;

pub const INITIAL_BALL_VELOCITY: glm::Vec2 = glm::Vec2::new(100.0, -350.0);
pub const BALL_RADIUS: f32 = 12.5;

#[derive(Debug, Clone)]
pub struct Ball {
    pub object: GameObject,
    // ball state
    pub radius: f32,
    pub stuck: bool,
}

impl Ball {
    pub fn new(position: glm::Vec2, radius: f32, stuck: bool) -> Self {
        Self {
            object: GameObject {
                position,
                size: glm::vec2(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0),
                velocity: INITIAL_BALL_VELOCITY,
                color: glm::vec3(1.0, 1.0, 1.0),
                rotation: 0.0,
                is_solid: false,
                destroyed: false,
                sprite_id: String::from("ball"),
            },

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

    pub fn position(&self) -> glm::Vec2 {
        self.object.position
    }

    pub fn set_x(&mut self, x: f32) {
        self.object.position.x = x;
    }
}
