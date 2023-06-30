use nalgebra_glm as glm;

use crate::game_object::GameObject;

const POWERUP_SIZE: glm::Vec2 = glm::Vec2::new(60.0, 20.0);
const POWERUP_VELOCITY: glm::Vec2 = glm::Vec2::new(0.0, 150.0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerUpType {
    Speed,
    Sticky,
    PassThrough,
    PadSizeIncrease,
    Confuse,
    Chaos,
}

#[derive(Debug, Clone)]
pub struct PowerUp {
    object: GameObject,
    pub r#type: PowerUpType,
    duration: f32,
    activated: bool,
}

impl PowerUp {
    pub fn new(
        r#type: PowerUpType,
        color: glm::Vec3,
        duration: f32,
        position: glm::Vec2,
        sprite_id: String,
    ) -> Self {
        Self {
            object: GameObject::new(position, POWERUP_SIZE, color, POWERUP_VELOCITY, sprite_id),
            r#type,
            duration,
            activated: false,
        }
    }
}
