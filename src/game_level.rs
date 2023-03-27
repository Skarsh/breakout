use std::{fs, path::Path};

use nalgebra_glm as glm;

use crate::{
    game_object::GameObject, graphics::sprite_renderer::SpriteRenderer,
    graphics::texture_manager::TextureManager,
};

#[derive(Debug)]
pub struct GameLevel {
    pub bricks: Vec<GameObject>,
}

impl GameLevel {
    pub fn load(
        &mut self,
        file: &Path,
        level_width: u32,
        level_height: u32,
        texture_manager: &TextureManager,
    ) {
        // clear old data
        self.bricks.clear();
        let mut tile_data = vec![];

        let contents = fs::read_to_string(file).expect("Should have been able to read the file");
        for line in contents.lines() {
            let mut char_line = vec![];
            for c in line.chars() {
                match c {
                    // 0 is empty space, but is still needed to make the
                    // width calculations for the level
                    '0' => char_line.push(c),
                    '1' => char_line.push(c),
                    '2' => char_line.push(c),
                    '3' => char_line.push(c),
                    '4' => char_line.push(c),
                    '5' => char_line.push(c),
                    _ => {}
                }
            }
            tile_data.push(char_line);
        }
        if tile_data.len() > 0 {
            self.init(tile_data, level_width, level_height, texture_manager);
        }
    }

    pub fn draw(&mut self, renderer: &mut SpriteRenderer, texture_manager: &TextureManager) {
        for tile in self.bricks.iter_mut() {
            if !tile.destroyed {
                let texture = texture_manager.get_texture(&tile.sprite_id());
                tile.draw(renderer, texture);
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        for tile in &self.bricks {
            if !tile.is_solid && !tile.destroyed {
                return false;
            }
        }
        true
    }

    fn init(
        &mut self,
        tile_data: Vec<Vec<char>>,
        level_width: u32,
        level_height: u32,
        texture_manager: &TextureManager,
    ) {
        let height = tile_data.len();
        let width = tile_data.get(0).unwrap().len();
        let unit_width = level_width as f32 / width as f32;
        let unit_height = level_height as f32 / height as f32;

        // intialize levels based on tile_data
        //for y in 0..height {
        for (y, _) in tile_data.iter().enumerate() {
            for (x, _) in tile_data[y].iter().enumerate() {
                let pos = glm::vec2(unit_width * x as f32, unit_height as f32 * y as f32);
                let size = glm::vec2(unit_width, unit_height);
                let velocity = glm::vec2(0.0, 0.0);
                let mut obj = GameObject {
                    position: pos,
                    size,
                    velocity,
                    ..Default::default()
                };
                match tile_data[y][x] {
                    '1' => {
                        obj.color = glm::vec3(0.8, 0.8, 0.7);
                        obj.is_solid = true;
                        obj.sprite_id = String::from("block_solid");
                        self.bricks.push(obj);
                    }
                    '2' => {
                        obj.color = glm::vec3(0.2, 0.6, 1.0);
                        obj.sprite_id = String::from("block");
                        self.bricks.push(obj);
                    }
                    '3' => {
                        obj.color = glm::vec3(0.0, 0.7, 0.0);
                        obj.sprite_id = String::from("block");
                        self.bricks.push(obj);
                    }
                    '4' => {
                        obj.color = glm::vec3(0.8, 0.8, 0.4);
                        obj.sprite_id = String::from("block");
                        self.bricks.push(obj);
                    }
                    '5' => {
                        obj.color = glm::vec3(1.0, 0.5, 0.0);
                        obj.sprite_id = String::from("block");
                        self.bricks.push(obj);
                    }
                    _ => {}
                }
            }
        }
    }
}
