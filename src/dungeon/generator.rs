use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::ChaCha20Rng;

use super::{Cell, DungeonGenerator, Map, TileKind};

#[derive(Clone, Copy, Debug)]
struct MapPosition {
    x: usize,
    y: usize,
}

const CEILING_HEIGHT: f32 = 2.0;

#[allow(dead_code)]
pub struct BorderedRoom;
impl DungeonGenerator for BorderedRoom {
    fn generate(&self, width: usize, height: usize) -> Map {
        let mut cells = Vec::with_capacity(width * height);
        let spawn = (width / 2, height / 2);
        let wall_cell = Cell {
            kind: TileKind::Wall,
            ceiling_height: CEILING_HEIGHT,
        };
        let floor_cell = Cell {
            kind: TileKind::Floor,
            ceiling_height: CEILING_HEIGHT,
        };
        for j in 0..height {
            for i in 0..width {
                if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                    cells.push(wall_cell);
                } else {
                    cells.push(floor_cell);
                }
            }
        }

        Map {
            cells,
            width,
            height,
            spawn,
        }
    }
}

const N_STEPS: usize = 100;

pub struct DrunkenWalk;
impl DungeonGenerator for DrunkenWalk {
    fn generate(&self, width: usize, height: usize) -> Map {
        let wall_cell = Cell {
            kind: TileKind::Wall,
            ceiling_height: CEILING_HEIGHT,
        };
        let floor_cell = Cell {
            kind: TileKind::Floor,
            ceiling_height: CEILING_HEIGHT,
        };
        let mut cells = vec![wall_cell; width * height];
        let mut pos = MapPosition {
            x: width / 2,
            y: height / 2,
        };
        let spawn = (pos.x, pos.y);
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        for _ in 0..N_STEPS {
            let index = pos.x + pos.y * width;
            cells[index] = floor_cell;
            let step_dir: u8 = rng.random_range(0..4);
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
            cells,
            width,
            height,
            spawn,
        }
    }
}
