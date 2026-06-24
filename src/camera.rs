use bevy::input::mouse::*;
use bevy::prelude::*;

use crate::state::Pause;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller.run_if(in_state(Pause::Running)));
    }
}
fn get_direction(keys: &ButtonInput<KeyCode>, transform: &Transform) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= *transform.right();
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }
    direction.normalize_or_zero()
}

pub fn camera_controller(
    mut query: Query<(&mut Transform, &mut CameraController)>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let (mut transform, mut controller) = query.single_mut().unwrap();

    let delta = mouse.delta;
    controller.yaw -= delta.x * controller.sensitivity;
    controller.pitch -= delta.y * controller.sensitivity;

    // pitch clamp to prevent flipping over the top
    controller.pitch = controller.pitch.clamp(-1.54, 1.54);
    transform.rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw)
        * Quat::from_axis_angle(Vec3::X, controller.pitch);
    let direction = get_direction(&keys, &transform);
    transform.translation += direction * controller.speed * time.delta_secs();
    // info!("yaw={:.3} pitch={:.3}", controller.yaw, controller.pitch);
}
