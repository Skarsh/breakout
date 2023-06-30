#![allow(dead_code)]

use std::{ops::Neg, path::Path};

use glfw::ffi::glfwGetTime;
use nalgebra_glm as glm;

use crate::{
    ball::{Ball, BALL_RADIUS, INITIAL_BALL_VELOCITY},
    game_level::GameLevel,
    game_object::GameObject,
    graphics::{post_processor::PostProcessor, shader_manager::ShaderManager},
    graphics::{texture_manager::TextureManager, Graphics},
    particle_generator::ParticleGenerator,
    powerup::PowerUp,
};

#[derive(Debug)]
enum GameState {
    Active,
    Menu,
    Win,
}

pub const PLAYER_SIZE: glm::Vec2 = glm::Vec2::new(100.0, 20.0);
pub const PLAYER_VELOCITY: f32 = 500.0;

#[derive(Debug)]
pub struct Game {
    state: GameState,
    pub keys: [bool; 1024],
    pub graphics: Graphics,
    levels: Vec<GameLevel>,
    level: u32,
    player: GameObject,
    ball: Ball,
    particle_generator: ParticleGenerator,
    effects: PostProcessor,
    shake_time: f32,
    powerups: Vec<PowerUp>,
}

impl Game {
    pub fn new(mut graphics: Graphics) -> Self {
        // load textures
        load_textures(&mut graphics.texture_manager);
        load_shaders(&mut graphics.shader_manager);

        let mut levels = vec![];
        load_levels(
            &mut levels,
            graphics.width,
            graphics.height,
            &graphics.texture_manager,
        );

        // Player
        let player_pos = glm::vec2(
            graphics.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            graphics.height as f32 - PLAYER_SIZE.y,
        );

        let player = GameObject {
            position: player_pos,
            size: PLAYER_SIZE,
            velocity: glm::vec2(0.0, 0.0),
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite_id: String::from("paddle"),
        };

        let ball_pos =
            player_pos + glm::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -BALL_RADIUS * 2.0);
        let ball = Ball::new(ball_pos, BALL_RADIUS, true);

        let mut particle_generator = ParticleGenerator::new(
            graphics.shader_manager.get_shader("particle").clone(),
            graphics.texture_manager.get_texture("particle").clone(),
            500,
        );
        particle_generator.init();

        let effects = PostProcessor::new(
            graphics.shader_manager.get_shader("postprocessing").clone(),
            graphics.width as i32,
            graphics.height as i32,
        );

        Self {
            state: GameState::Active,
            keys: [false; 1024],
            graphics,
            levels,
            level: 0,
            player,
            ball,
            particle_generator,
            effects,
            shake_time: 0.0,
            powerups: vec![],
        }
    }

    pub fn init(&mut self) {
        // configure shaders
        let projection = glm::ortho(
            0.0,
            self.graphics.width as f32,
            self.graphics.height as f32,
            0.0,
            -1.0,
            1.0,
        );
        self.graphics
            .shader_manager
            .get_shader("sprite")
            .use_program()
            .set_int("image\0", 0);

        self.graphics
            .shader_manager
            .get_shader("sprite")
            .set_mat4("projection\0", &projection);

        self.graphics
            .shader_manager
            .get_shader("particle")
            .use_program()
            .set_int("sprite\0", 0);

        self.graphics
            .shader_manager
            .get_shader("particle")
            .set_mat4("projection\0", &projection);
    }

    pub fn process_input(&mut self, dt: f64) {
        match self.state {
            GameState::Menu => {}
            GameState::Win => {}
            GameState::Active => {
                let velocity = PLAYER_VELOCITY * dt as f32;

                // move player paddle
                if self.keys[glfw::Key::A as usize] && self.player.position.x >= 0.0 {
                    self.player.position.x -= velocity;
                    if self.ball.stuck {
                        self.ball.set_x(self.ball.position().x - velocity);
                    }
                }

                if self.keys[glfw::Key::D as usize]
                    && self.player.position.x <= self.graphics.width as f32 - self.player.size.x
                {
                    self.player.position.x += velocity;
                    if self.ball.stuck {
                        self.ball.set_x(self.ball.position().x + velocity);
                    }
                }

                if self.keys[glfw::Key::Space as usize] {
                    self.ball.stuck = false;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.ball.move_ball(dt as f32, self.graphics.width);
        self.do_collisions();
        self.particle_generator.update(
            dt as f32,
            &self.ball.object,
            2,
            glm::vec2(self.ball.radius / 2.0, self.ball.radius / 2.0),
        );
        if self.ball.position().y >= self.graphics.height as f32 {
            self.reset_level();
            self.reset_player();
        }

        if self.shake_time > 0.0 {
            self.shake_time -= dt as f32;
            if self.shake_time <= 0.0 {
                self.effects.shake = false;
            }
        }
    }

    pub fn render(&mut self) {
        match self.state {
            GameState::Active => {
                self.effects.begin_render();
                self.graphics.render();
                if let Some(level) = self.levels.get_mut(self.level as usize) {
                    level.draw(
                        &mut self.graphics.sprite_renderer,
                        &self.graphics.texture_manager,
                    );
                }
                self.player.draw(
                    &mut self.graphics.sprite_renderer,
                    self.graphics.texture_manager.get_texture("paddle"),
                );
                self.particle_generator.draw();
                self.ball.draw(
                    &mut self.graphics.sprite_renderer,
                    self.graphics.texture_manager.get_texture("ball"),
                );
                self.effects.end_render();
                unsafe {
                    self.effects.render(glfwGetTime() as f32);
                }
            }
            _ => panic!("Illegal state"),
        }
    }

    pub fn clear(&mut self) {
        self.graphics.clear();
    }

    fn reset_level(&mut self) {
        match self.level {
            0 => self.levels.get_mut(0).unwrap().load(
                Path::new("resources/levels/one.lvl"),
                self.graphics.width,
                self.graphics.height / 2,
                &self.graphics.texture_manager,
            ),
            1 => self.levels.get_mut(1).unwrap().load(
                Path::new("resources/levels/two.lvl"),
                self.graphics.width,
                self.graphics.height / 2,
                &self.graphics.texture_manager,
            ),
            2 => self.levels.get_mut(2).unwrap().load(
                Path::new("resources/levels/three.lvl"),
                self.graphics.width,
                self.graphics.height / 2,
                &self.graphics.texture_manager,
            ),
            3 => self.levels.get_mut(3).unwrap().load(
                Path::new("resources/levels/three.lvl"),
                self.graphics.width,
                self.graphics.height / 2,
                &self.graphics.texture_manager,
            ),
            _ => eprintln!("Illegal level!"),
        }
    }

    fn reset_player(&mut self) {
        self.player.size = PLAYER_SIZE;
        self.player.position = glm::vec2(
            (self.graphics.width as f32 / 2.0) - (self.player.size.x / 2.0),
            self.graphics.height as f32 - PLAYER_SIZE.y,
        );
        self.ball.reset(
            self.player.position
                + glm::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -(BALL_RADIUS * 2.0)),
            INITIAL_BALL_VELOCITY,
        );
    }

    fn spawn_powerups(&self, block: &GameObject) {
        unimplemented!()
    }

    fn update_powerups(&self, dt: f32) {
        unimplemented!()
    }

    fn do_collisions(&mut self) {
        for brick in &mut self.levels[self.level as usize].bricks {
            if !brick.destroyed {
                let collision = check_collision_circle(&self.ball, brick);

                if collision.0 {
                    if !brick.is_solid {
                        brick.destroyed = true;
                    } else {
                        self.shake_time = 0.05;
                        self.effects.shake = true;
                    }

                    let dir = collision.1;
                    let diff_vector = collision.2;

                    // Horizontal collision
                    if dir == Direction::Left || dir == Direction::Right {
                        self.ball.object.velocity.x *= -1.0;

                        // relocate
                        let penetration = self.ball.radius - diff_vector.x.abs();
                        if dir == Direction::Left {
                            // move ball to the right
                            self.ball.position().x += penetration;
                        } else {
                            self.ball.position().x -= penetration;
                        }
                    } else {
                        // vertical collision
                        self.ball.object.velocity.y *= -1.0;
                        let penetration = self.ball.radius - diff_vector.y.abs();
                        if dir == Direction::Up {
                            // move ball back up
                            self.ball.position().y -= penetration;
                        } else {
                            self.ball.position().y += penetration;
                        }
                    }
                }
            }
        }
        let result = check_collision_circle(&self.ball, &self.player);
        if !self.ball.stuck && result.0 {
            // check where it hit the board, and change directin accordingly
            let center_board = self.player.position.x + self.player.size.x / 2.0;
            let distance = (self.ball.position().x + self.ball.radius) - center_board;
            let percentage = distance / (self.player.size.x / 2.0);

            // move accordingly
            let strength = 2.0;
            let old_velocity = self.ball.object.velocity;
            self.ball.object.velocity.x = INITIAL_BALL_VELOCITY.x * percentage * strength;
            self.ball.object.velocity =
                glm::normalize(&self.ball.object.velocity) * glm::length(&old_velocity);
            self.ball.object.velocity.y = -1.0 * self.ball.object.velocity.y.abs();
        }
    }
}

fn load_shaders(shader_manager: &mut ShaderManager) {
    shader_manager.load_shader(
        Path::new("shaders/particle.vs"),
        Path::new("shaders/particle.frag"),
        None,
        "particle".to_string(),
    );

    shader_manager.load_shader(
        Path::new("shaders/post_processing.vs"),
        Path::new("shaders/post_processing.frag"),
        None,
        "postprocessing".to_string(),
    );
}

fn load_textures(texture_manager: &mut TextureManager) {
    texture_manager.load_texture(
        Path::new("resources/textures/background.jpg"),
        false,
        "background",
    );

    texture_manager.load_texture(
        Path::new("resources/textures/awesomeface.png"),
        true,
        "ball",
    );

    texture_manager.load_texture(Path::new("resources/textures/block.png"), false, "block");

    texture_manager.load_texture(
        Path::new("resources/textures/block_solid.png"),
        false,
        "block_solid",
    );

    texture_manager.load_texture(Path::new("resources/textures/paddle.png"), true, "paddle");

    texture_manager.load_texture(
        Path::new("resources/textures/particle.png"),
        true,
        "particle",
    );
}

fn load_levels(
    levels: &mut Vec<GameLevel>,
    width: u32,
    height: u32,
    texture_manager: &TextureManager,
) {
    // load levels
    let mut one = GameLevel { bricks: vec![] };
    one.load(
        Path::new("resources/levels/one.lvl"),
        width,
        height / 2,
        texture_manager,
    );
    levels.push(one);
    let mut two = GameLevel { bricks: vec![] };
    two.load(
        Path::new("resources/levels/two.lvl"),
        width,
        height / 2,
        texture_manager,
    );
    let mut three = GameLevel { bricks: vec![] };
    three.load(
        Path::new("resources/levels/three.lvl"),
        width,
        height / 2,
        texture_manager,
    );
    let mut four = GameLevel { bricks: vec![] };
    four.load(
        Path::new("resources/levels/four.lvl"),
        width,
        height / 2,
        texture_manager,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Collision(bool, Direction, glm::Vec2);

fn check_collision_circle(one: &Ball, two: &GameObject) -> Collision {
    let center = glm::vec2(one.position().x + one.radius, one.position().y + one.radius);
    let aabb_half_extents = glm::vec2(two.size.x / 2.0, two.size.y / 2.0);
    let aabb_center = glm::vec2(
        two.position.x + aabb_half_extents.x,
        two.position.y + aabb_half_extents.y,
    );

    let mut difference = center - aabb_center;
    let clamped = glm::clamp_vec(&difference, &aabb_half_extents.neg(), &aabb_half_extents);

    let closest = aabb_center + clamped;

    difference = closest - center;
    if glm::length(&difference) < one.radius {
        Collision(true, vector_direction(difference), difference)
    } else {
        Collision(false, Direction::Up, glm::vec2(0.0, 0.0))
    }
}

fn vector_direction(target: glm::Vec2) -> Direction {
    let compass: [glm::Vec2; 4] = [
        glm::vec2(0.0, 1.0),  // up
        glm::vec2(1.0, 0.0),  // right
        glm::vec2(0.0, -1.0), // down
        glm::vec2(-1.0, 0.0), // left
    ];

    let mut max = 0.0;
    let mut best_match = Direction::Up;

    #[allow(clippy::needless_range_loop)]
    for i in 0..compass.len() {
        let dot_product = glm::dot(&glm::normalize(&target), &compass[i]);

        if dot_product.is_nan() {
            continue;
        }

        if dot_product > max {
            max = dot_product;
            match i {
                0 => best_match = Direction::Up,
                1 => best_match = Direction::Right,
                2 => best_match = Direction::Down,
                3 => best_match = Direction::Left,
                _ => eprintln!("Illegal direction!"),
            }
        }
    }

    best_match
}
