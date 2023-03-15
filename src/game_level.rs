use std::{fs, path::Path};

use nalgebra_glm as glm;

use crate::{
    game_object::GameObject,
    sprite_renderer::SpriteRenderer,
    texture::Texture2D,
    texture_manager::{self, TextureManager},
};

#[derive(Debug)]
pub struct GameLevel<'a> {
    pub bricks: Vec<GameObject<'a>>,
}

impl<'a> GameLevel<'a> {
    pub fn load(
        &mut self,
        file: &Path,
        level_width: u32,
        level_height: u32,
        texture_manager: &'a TextureManager,
    ) {
        // clear old data
        self.bricks.clear();
        let mut tile_data = vec![vec![]];

        let contents = fs::read_to_string(file).expect("Should have been able to read the file");
        for line in contents.lines() {
            let row = line.as_bytes().to_vec();
            tile_data.push(row);
        }
        if tile_data.len() > 0 {
            self.init(tile_data, level_width, level_height, texture_manager);
        }
    }

    pub fn draw(&mut self, renderer: &mut SpriteRenderer) {
        for tile in self.bricks.iter_mut() {
            if !tile.destroyed {
                tile.draw(renderer)
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
        tile_data: Vec<Vec<u8>>,
        level_width: u32,
        level_height: u32,
        texture_manager: &'a TextureManager,
    ) {
        let height = tile_data.len();
        let width = tile_data.get(0).unwrap().len();
        let unit_width = level_width as f32 / width as f32;
        let unit_height = level_height as f32 / height as f32;

        // intialize levels based on tile_data
        for y in 0..height {
            for x in 0..width {
                // Solid
                if tile_data[y][x] == 1 {
                    let pos = glm::vec2(unit_width * x as f32, unit_height * y as f32);
                    let size = glm::vec2(unit_width, unit_height);
                    let mut obj = GameObject::new(
                        pos,
                        size,
                        glm::vec3(0.8, 0.8, 0.7),
                        glm::vec2(0.0, 0.0),
                        texture_manager.get_texture("block_solid"),
                    );
                    obj.is_solid = true;
                    self.bricks.push(obj);
                } else if tile_data[y][x] > 1 {
                    let mut color = glm::vec3(1.0, 1.0, 1.0);
                    match tile_data[y][x] {
                        2 => color = glm::vec3(0.2, 0.6, 1.0),
                        3 => color = glm::vec3(0.0, 0.7, 0.0),
                        4 => color = glm::vec3(0.8, 0.8, 0.4),
                        5 => color = glm::vec3(1.0, 0.5, 0.0),
                        _ => panic!("illegal level tile value"),
                    }

                    let pos = glm::vec2(unit_width * x as f32, unit_height * y as f32);
                    let size = glm::vec2(unit_width, unit_height);
                    self.bricks.push(GameObject::new(
                        pos,
                        size,
                        color,
                        glm::vec2(0.0, 0.0),
                        texture_manager.get_texture("block"),
                    ));
                }
            }
        }
    }
}
