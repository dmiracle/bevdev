use bevy::prelude::*;

#[derive(Component)]
struct CameraController {
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}


