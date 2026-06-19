mod generator;

use bevy::prelude::*;

use crate::collision::Collider;
use generator::{BorderedRoom, DrunkenWalk};
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_dungeon);
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Floor,
    Wall,
}

const TILE_SIZE: f32 = 2.0;

struct Map {
    tiles: Vec<Tile>, // flat row major: index y * with + x
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, i: usize, j: usize) -> &Tile {
        &self.tiles[i + j * self.width]
    }
}

trait DungeonGenerator {
    fn generate(&self, width: usize, height: usize) -> Map;
}

fn setup_dungeon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = DrunkenWalk;
    let map = generator.generate(20, 15);

    let floor_mesh = meshes.add(Plane3d::default().mesh().size(TILE_SIZE, TILE_SIZE));
    let floor_mat = materials.add(Color::srgb(0.4, 0.1, 0.8));
    let cube = Cuboid::from_size(Vec3::splat(TILE_SIZE));
    let mesh = meshes.add(cube);
    let mat = materials.add(Color::srgb(0.5, 0.5, 0.6));

    for j in 0..map.height {
        for i in 0..map.width {
            if matches!(map.get(i, j), Tile::Wall) {
                let pos = Vec3::new(i as f32 * TILE_SIZE, TILE_SIZE / 2.0, j as f32 * TILE_SIZE);
                commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_translation(pos),
                    Collider {
                        half_extents: cube.half_size,
                    },
                ));
            }
            let pos = Vec3::new(i as f32 * TILE_SIZE, 0.0, j as f32 * TILE_SIZE);
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(floor_mat.clone()),
                Transform::from_translation(pos),
            ));
        }
    }
}
