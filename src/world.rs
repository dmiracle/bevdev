use bevy::prelude::*;

use crate::camera::CameraController;
use crate::collision::Collider;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));
    let floor_mat = materials.add(Color::srgb(0.3, 0.5, 0.3));
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cube_mat = materials.add(Color::srgb(1.0, 0.0, 0.0));
    let wall_mat = materials.add(Color::srgb(1.0, 0.0, 1.0));

    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_mat),
        Transform::from_xyz(0.0, 0.5, 0.5),
        Collider {
            half_extents: Vec3::new(0.5, 0.5, 0.5),
        },
    ));
    // walls
    for (size, pos) in [
        (Vec3::new(10.0, 4.0, 0.5), Vec3::new(0.0, 2.0, -5.0)),
        (Vec3::new(0.5, 4.0, 10.0), Vec3::new(-5.0, 2.0, 0.0)),
        (Vec3::new(10.0, 4.0, 0.5), Vec3::new(0.0, 2.0, 5.0)),
        (Vec3::new(0.5, 4.0, 10.0), Vec3::new(5.0, 2.0, 0.0)),
    ] {
        let wall_boid = Cuboid::from_size(size);
        let wall_mesh = meshes.add(wall_boid);
        commands.spawn((
            Mesh3d(wall_mesh),
            MeshMaterial3d(wall_mat.clone()),
            Transform::from_translation(pos),
            Collider {
                half_extents: wall_boid.half_size,
            },
        ));
    }
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0),
        CameraController {
            yaw: -0.4,
            pitch: -0.4,
            speed: 5.0,
            sensitivity: 0.003,
        },
    ));
}
