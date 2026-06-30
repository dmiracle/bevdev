use bevy::prelude::*;

use crate::camera::CameraController;
use crate::state::GameState;
use crate::collision::Collider;

#[derive(Component)]
pub struct DebugText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_colliders)
            .add_systems(Startup, setup_debug)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_debug(mut commands: Commands) {
    // Debugger
    commands.spawn((
        Text::new("pos:"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        DebugText,
    ));
}

fn update_debug_text(
    player: Query<(&Transform, &CameraController)>,
    mut text: Query<&mut Text, With<DebugText>>,
) {
    let (pos, con) = player.single().unwrap();
    text.single_mut().unwrap().0 = format!(
        "pos: {:.2}\npitch: {:.2}, yaw: {:.2}",
        pos.translation, con.pitch, con.yaw
    );
}

fn draw_colliders(mut gizmos: Gizmos, query: Query<(&Transform, &Collider)>) {
    for (transform, collider) in &query {
        let center = transform.translation + collider.offset;
        let size = collider.half_extents * 2.0;
        gizmos.cube(
            Transform::from_translation(center).with_scale(size),
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}
