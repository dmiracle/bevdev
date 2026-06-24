mod generator;

use bevy::prelude::*;

use crate::collision::Collider;
use crate::state::GameState;
use generator::DrunkenWalk;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_dungeon)
            .add_systems(OnExit(GameState::InGame), cleanup_dungeon);
    }
}

#[derive(Component)]
struct DungeonTile;

#[derive(Clone, Copy)]
pub enum Tile {
    Floor,
    Wall,
}

pub const TILE_SIZE: f32 = 2.0;

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Tile>, // flat row major: index y * with + x
    pub width: usize,
    pub height: usize,
    pub spawn: (usize, usize),
}

impl Map {
    fn get(&self, i: usize, j: usize) -> &Tile {
        &self.tiles[i + j * self.width]
    }
}

trait DungeonGenerator {
    fn generate(&self, width: usize, height: usize) -> Map;
}

pub fn setup_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let generator = DrunkenWalk;
    let map = generator.generate(20, 15);
    let wall_scene: Handle<Scene> = asset_server.load("models/wall.glb#Scene0");
    let floor_scene: Handle<Scene> = asset_server.load("models/floor.glb#Scene0");

    for j in 0..map.height {
        for i in 0..map.width {
            if matches!(map.get(i, j), Tile::Wall) {
                let pos = Vec3::new(i as f32 * TILE_SIZE, 0.0, j as f32 * TILE_SIZE);
                commands.spawn((
                    SceneRoot(wall_scene.clone()),
                    Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
                    Collider {
                        half_extents: Vec3::splat(TILE_SIZE / 2.0),
                        offset: Vec3::new(0.0, 1.0, 0.0),
                    },
                    DungeonTile,
                ));
            }
            let pos = Vec3::new(i as f32 * TILE_SIZE, 0.0, j as f32 * TILE_SIZE);
            commands.spawn((
                SceneRoot(floor_scene.clone()),
                Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
                DungeonTile,
            ));
        }
    }

    commands.insert_resource(map);
}

fn cleanup_dungeon(mut commands: Commands, tiles: Query<Entity, With<DungeonTile>>) {
    for entity in &tiles {
        commands.entity(entity).despawn();
    }
}
