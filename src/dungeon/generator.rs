use bevy::prelude::*;
use rand::Rng;

use super::{Map, Tile, DungeonGenerator};

pub struct BorderedRoom;
impl DungeonGenerator for BorderedRoom {
    fn generate(&self, width: usize, height: usize) -> Map {
        let mut tiles = Vec::with_capacity(width * height);

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
        }
    }
}

pub struct DrunkenWalk;
impl DungeonGenerator for DrunkenWalk {
    fn generate(&self, width: usize, height: usize) -> Map {
        let tile = Tile::Wall;
        let mut tiles = vec![tile.clone(); (width * height)];

        Map {
            tiles,
            width,
            height,
        }
    }
}
