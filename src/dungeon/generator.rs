use bevy::prelude::*;

use super::{DungeonGenerator, Map, Tile};

#[derive(Clone, Copy, Debug)]
struct MapPosition {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
pub struct BorderedRoom;
impl DungeonGenerator for BorderedRoom {
    fn generate(&self, width: usize, height: usize) -> Map {
        let mut tiles = Vec::with_capacity(width * height);
        let spawn = (width / 2, height / 2);
        for j in 0..height {
            for i in 0..width {
                if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                    tiles.push(Tile::Wall);
                } else {
                    tiles.push(Tile::Floor);
                }
            }
        }

        Map {
            tiles,
            width,
            height,
            spawn,
        }
    }
}

const N_STEPS: usize = 1000;

pub struct DrunkenWalk;
impl DungeonGenerator for DrunkenWalk {
    fn generate(&self, width: usize, height: usize) -> Map {
        let mut tiles = vec![Tile::Wall; width * height];
        let mut pos = MapPosition {
            x: width / 2,
            y: height / 2,
        };
        let spawn = (pos.x, pos.y);
        for _ in 0..N_STEPS {
            let index = pos.x + pos.y * width;
            tiles[index] = Tile::Floor;
            let step_dir: u8 = rand::random_range(0..4);
            if step_dir == 0 && pos.x > 1 {
                pos.x -= 1;
            } else if step_dir == 1 && pos.y < height - 2 {
                pos.y += 1;
            } else if step_dir == 2 && pos.x < width - 2 {
                pos.x += 1;
            } else if pos.y > 1 {
                pos.y -= 1;
            }
        }
        Map {
            tiles,
            width,
            height,
            spawn,
        }
    }
}
