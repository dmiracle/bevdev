mod generator;

use crate::collision::Collider;
use crate::state::GameState;
use bevy::prelude::*;
use generator::DrunkenWalk;
use std::f32::consts::PI;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_dungeon)
            .add_systems(OnExit(GameState::InGame), cleanup_dungeon);
    }
}

#[derive(Component)]
struct DungeonTile;

#[derive(Clone, Copy, Debug)]
pub enum TileKind {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    kind: TileKind,
    ceiling_height: f32,
}

#[derive(Resource)]
pub struct Map {
    pub cells: Vec<Cell>, // flat row major: index y * with + x
    pub width: usize,
    pub height: usize,
    pub spawn: (usize, usize),
}

impl Map {
    fn get_cell(&self, i: usize, j: usize) -> &Cell {
        &self.cells[i + j * self.width]
    }
    fn get_tile_kind(&self, i: usize, j: usize) -> &TileKind {
        &self.cells[i + j * self.width].kind
    }
}

trait DungeonGenerator {
    fn generate(&self, width: usize, height: usize) -> Map;
}

pub const TILE_SIZE: f32 = 2.0;
pub const WALL_OFFSET: f32 = 0.0;
pub fn setup_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = DrunkenWalk;
    let map = generator.generate(20, 15);
    let wall_scene: Handle<Scene> = asset_server.load("models/template-wall.glb#Scene0");
    let floor_scene: Handle<Scene> = asset_server.load("models/template-floor.glb#Scene0");
    let ceiling_mesh = meshes.add(Plane3d::default().mesh().size(TILE_SIZE, TILE_SIZE));
    let ceiling_mat = materials.add(Color::srgb(0.8, 0.0, 0.9));
    for j in 0..map.height {
        for i in 0..map.width {
            let dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            if matches!(map.get_tile_kind(i, j), TileKind::Wall) {
                let pos = Vec3::new(i as f32 * TILE_SIZE, 0.0, i as f32 * TILE_SIZE);
                gizmos
                commands.spawn((
                        Transform::from_translation(pos),
                        Collider {
                            half_extents: Vec3::splat(TILE_SIZE / 2.0),
                            offset: Vec3::new(0.0, 1.0, 0.0),
                        },
                        DungeonTile,
                        ));
            }
            if matches!(map.get_tile_kind(i, j), TileKind::Floor) {
                for d in 0..4 {
                    let (dx, dy) = dxy[d];
                    let (x, y) = (i as isize + dx, j as isize + dy);
                    if x >= 0 && y >= 0 && x < map.width as isize && y < map.height as isize {
                        if matches!(map.get_tile_kind(x as usize, y as usize), TileKind::Wall) {
                            let x_pos =
                                dx as f32 * (TILE_SIZE / 2.0 + WALL_OFFSET) + i as f32 * TILE_SIZE;
                            let y_pos =
                                dy as f32 * (TILE_SIZE / 2.0 + WALL_OFFSET) + j as f32 * TILE_SIZE;
                            let pos = Vec3::new(x_pos, 0.0, y_pos);
                            let dir = Vec3::new(0.0, 1.0, 0.0);
                            let rot = Quat::from_axis_angle(dir, d as f32 * PI / 2.0);
                            println!("{:?}", rot);
                            commands.spawn((
                                SceneRoot(wall_scene.clone()),
                                Transform::from_translation(pos)
                                    .with_scale(Vec3::splat(0.5))
                                    .with_rotation(rot),
                                DungeonTile,
                            ));
                        }
                    }
                }
            }
            let ceiling_height = map.get_cell(i, j).ceiling_height;
            let pos = Vec3::new(
                i as f32 * TILE_SIZE,
                ceiling_height * 5.0,
                j as f32 * TILE_SIZE,
            );
            commands.spawn((
                Mesh3d(ceiling_mesh.clone()),
                MeshMaterial3d(ceiling_mat.clone()),
                Transform::from_translation(pos),
                DungeonTile,
            ));
        }
    }

    commands.insert_resource(map);
}

fn cleanup_dungeon(mut commands: Commands, cells: Query<Entity, With<DungeonTile>>) {
    for entity in &cells {
        commands.entity(entity).despawn();
    }
}
