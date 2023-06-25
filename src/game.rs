use std::{ops::Neg, path::Path};

use nalgebra_glm as glm;

use crate::{
    ball::{Ball, BALL_RADIUS, INITIAL_BALL_VELOCITY},
    game_level::GameLevel,
    game_object::GameObject,
    graphics::sprite_renderer::SpriteRenderer,
    graphics::Graphics,
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
    player: Option<GameObject>,
    ball: Option<Ball>,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        Self {
            state: GameState::Active,
            keys: [false; 1024],
            graphics,
            levels: vec![],
            level: 0,
            player: None,
            ball: None,
        }
    }

    pub fn init(&mut self) {
        // load shaders
        let shader = self.graphics.shader_manager.load_shader(
            Path::new("shaders/sprite.vs"),
            Path::new("shaders/sprite.frag"),
            None,
            "sprite".to_string(),
        );

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

        // set render-specific controls
        let _renderer = SpriteRenderer::new(shader);

        // load textures
        self.graphics.texture_manager.load_texture(
            Path::new("resources/textures/background.jpg"),
            false,
            "background",
        );

        self.graphics.texture_manager.load_texture(
            Path::new("resources/textures/awesomeface.png"),
            true,
            "ball",
        );

        self.graphics.texture_manager.load_texture(
            Path::new("resources/textures/block.png"),
            false,
            "block",
        );

        self.graphics.texture_manager.load_texture(
            Path::new("resources/textures/block_solid.png"),
            false,
            "block_solid",
        );

        self.graphics.texture_manager.load_texture(
            Path::new("resources/textures/paddle.png"),
            true,
            "paddle",
        );

        // Player
        let player_pos = glm::vec2(
            self.graphics.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.graphics.height as f32 - PLAYER_SIZE.y,
        );

        self.player = Some(GameObject {
            position: player_pos,
            size: PLAYER_SIZE,
            velocity: glm::vec2(0.0, 0.0),
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            is_solid: false,
            destroyed: false,
            sprite_id: String::from("paddle"),
        });

        // Ball
        let ball_pos = glm::vec2(
            self.graphics.width as f32 / 2.0 - BALL_RADIUS,
            self.graphics.height as f32 - PLAYER_SIZE.y * 2.0,
        );

        self.ball = Some(Ball::new(ball_pos, BALL_RADIUS, true));

        // load levels
        let mut one = GameLevel { bricks: vec![] };
        one.load(
            Path::new("resources/levels/one.lvl"),
            self.graphics.width,
            self.graphics.height / 2,
            &self.graphics.texture_manager,
        );
        self.levels.push(one);
        let mut two = GameLevel { bricks: vec![] };
        two.load(
            Path::new("resources/levels/two.lvl"),
            self.graphics.width,
            self.graphics.height / 2,
            &self.graphics.texture_manager,
        );
        let mut three = GameLevel { bricks: vec![] };
        three.load(
            Path::new("resources/levels/three.lvl"),
            self.graphics.width,
            self.graphics.height / 2,
            &self.graphics.texture_manager,
        );
        let mut four = GameLevel { bricks: vec![] };
        four.load(
            Path::new("resources/levels/four.lvl"),
            self.graphics.width,
            self.graphics.height / 2,
            &self.graphics.texture_manager,
        );
    }

    pub fn process_input(&mut self, dt: f64) {
        match self.state {
            GameState::Menu => {}
            GameState::Win => {}
            GameState::Active => {
                let velocity = PLAYER_VELOCITY * dt as f32;
                // move player paddle
                if self.keys[glfw::Key::A as usize] {
                    if let Some(ref mut player) = self.player {
                        if player.position.x >= 0.0 {
                            player.position.x -= velocity;
                            if let Some(ref mut ball) = self.ball {
                                if ball.stuck {
                                    ball.set_x(ball.position().x - velocity);
                                }
                            }
                        }
                    }
                }
                if self.keys[glfw::Key::D as usize] {
                    if let Some(ref mut player) = self.player {
                        if player.position.x <= self.graphics.width as f32 - player.size.x {
                            player.position.x += velocity;
                            if let Some(ref mut ball) = self.ball {
                                if ball.stuck {
                                    ball.set_x(ball.position().x + velocity);
                                }
                            }
                        }
                    }
                }

                if self.keys[glfw::Key::Space as usize] {
                    if let Some(ref mut ball) = self.ball {
                        ball.stuck = false;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        if let Some(ref mut ball) = self.ball {
            ball.move_ball(dt as f32, self.graphics.width);
        }

        self.do_collisions();
    }

    pub fn render(&mut self) {
        match self.state {
            GameState::Active => {
                self.graphics.render();
                if let Some(level) = self.levels.get_mut(self.level as usize) {
                    level.draw(
                        &mut self.graphics.sprite_renderer,
                        &self.graphics.texture_manager,
                    );
                }
                if let Some(player) = &self.player {
                    player.draw(
                        &mut self.graphics.sprite_renderer,
                        &self.graphics.texture_manager.get_texture("paddle"),
                    );
                }
                if let Some(ball) = &self.ball {
                    ball.draw(
                        &mut self.graphics.sprite_renderer,
                        &self.graphics.texture_manager.get_texture("ball"),
                    );
                }
            }
            _ => panic!("Illegal state"),
        }
    }

    pub fn clear(&mut self) {
        self.graphics.clear();
    }

    fn do_collisions(&mut self) {
        for brick in &mut self.levels[self.level as usize].bricks {
            if !brick.destroyed {
                if let Some(ref mut ball) = self.ball {
                    let collision = check_collision_circle(&ball, brick);

                    if collision.0 {
                        if !brick.is_solid {
                            brick.destroyed = true;
                        }

                        let dir = collision.1;
                        let diff_vector = collision.2;

                        // Horizontal collision
                        if dir == Direction::Left || dir == Direction::Right {
                            ball.object.velocity.x *= -1.0;

                            // relocate
                            let penetration = ball.radius - diff_vector.x.abs();
                            if dir == Direction::Left {
                                // move ball to the right
                                ball.position().x += penetration;
                            } else {
                                ball.position().x -= penetration;
                            }
                        } else {
                            // vertical collision
                            ball.object.velocity.y *= -1.0;
                            let penetration = ball.radius - diff_vector.y.abs();
                            if dir == Direction::Up {
                                // move ball back up
                                ball.position().y -= penetration;
                            } else {
                                ball.position().y += penetration;
                            }
                        }
                    }

                    if let Some(ref player) = self.player {
                        let result = check_collision_circle(&ball, &player);
                        if !ball.stuck && result.0 {
                            // check where it hit the board, and change directin accordingly
                            let center_board = player.position.x + player.size.x / 2.0;
                            let distance = (ball.position().x + ball.radius) - center_board;
                            let percentage = distance / (player.size.x / 2.0);

                            // move accordingly
                            let strength = 2.0;
                            let old_velocity = ball.object.velocity;
                            ball.object.velocity.x =
                                INITIAL_BALL_VELOCITY.x * percentage * strength;
                            ball.object.velocity.y = -ball.object.velocity.y;
                            ball.object.velocity =
                                glm::normalize(&ball.object.velocity) * glm::length(&old_velocity);
                        }
                    }
                }
            }
        }
    }
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
    let mut best_match = None;
    for i in 0..4 {
        let dot_product = glm::dot(&glm::normalize(&target), &compass[i]);
        if dot_product > max {
            max = dot_product;
            match i {
                0 => best_match = Some(Direction::Up),
                1 => best_match = Some(Direction::Right),
                2 => best_match = Some(Direction::Down),
                3 => best_match = Some(Direction::Left),
                _ => eprintln!("Illegal direction!"),
            }
        }
        println!("i: {}, dot_product: {}, max: {}", i, dot_product, max);
    }
    best_match.unwrap()
}
