use bevy::prelude::*;

use crate::camera::{CameraController, MovementMode};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
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
            mode: MovementMode::Walk,
        },
    ));
}
