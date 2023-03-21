use std::path::Path;

use nalgebra_glm as glm;

use crate::{game_level::GameLevel, graphics::Graphics, sprite_renderer::SpriteRenderer};

#[derive(Debug)]
enum GameState {
    Active,
    Menu,
    Win,
}

#[derive(Debug)]
pub struct Game<'a> {
    state: GameState,
    pub keys: [bool; 1024],
    pub graphics: Graphics,
    levels: Vec<GameLevel<'a>>,
    level: u32,
}

impl<'a> Game<'a> {
    pub fn new(graphics: Graphics) -> Self {
        Self {
            state: GameState::Active,
            keys: [false; 1024],
            graphics,
            levels: vec![],
            level: 0,
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
        let renderer = SpriteRenderer::new(shader);

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

        // load levels
        //let mut one = GameLevel {bricks: vec![]};
        //one.load(Path::new("levels/one.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
        //let mut two = GameLevel {bricks: vec![]};
        //two.load(Path::new("levels/two.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
        //let mut three = GameLevel {bricks: vec![]};
        //three.load(Path::new("levels/three.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
        //let mut four = GameLevel {bricks: vec![]};
        //three.load(Path::new("levels/four.lvl"), self.graphics.width, self.graphics.height / 2, &self.graphics.texture_manager);
    }

    pub fn process_input(&mut self, dt: f64) {}

    pub fn update(&mut self, dt: f64) {}

    pub fn render(&mut self) {
        match self.state {
            GameState::Active => {
                self.graphics.render();
                if let Some(level) = self.levels.get_mut(self.level as usize) {
                    level.draw(&mut self.graphics.sprite_renderer);
                }
            }
            _ => panic!("Illegal state"),
        }
    }

    pub fn clear(&mut self) {
        self.graphics.clear();
    }
}
