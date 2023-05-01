use std::path::Path;

use nalgebra_glm as glm;

use crate::{
    ball::Ball, game_level::GameLevel, game_object::GameObject,
    graphics::sprite_renderer::SpriteRenderer, graphics::Graphics,
};

#[derive(Debug)]
enum GameState {
    Active,
    Menu,
    Win,
}

const PLAYER_SIZE: glm::Vec2 = glm::Vec2::new(100.0, 20.0);
const PLAYER_VELOCITY: f32 = 500.0;
const INITIAL_BALL_VELOCITY: glm::Vec2 = glm::Vec2::new(100.0, -350.0);
const BALL_RADIUS: f32 = 12.5;

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
            "face",
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

        // load levels
        let mut one = GameLevel { bricks: vec![] };
        one.load(
            Path::new("resources/levels/one.lvl"),
            self.graphics.width,
            self.graphics.height / 2,
            &self.graphics.texture_manager,
        );
        self.levels.push(one);
        //let mut two = GameLevel {bricks: vec![]};
        //two.load(Path::new("levels/two.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
        //let mut three = GameLevel {bricks: vec![]};
        //three.load(Path::new("levels/three.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
        //let mut four = GameLevel {bricks: vec![]};
        //three.load(Path::new("levels/four.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
    }

    pub fn process_input(&mut self, _dt: f64) {}

    pub fn update(&mut self, _dt: f64) {}

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
            }
            _ => panic!("Illegal state"),
        }
    }

    pub fn clear(&mut self) {
        self.graphics.clear();
    }
}
