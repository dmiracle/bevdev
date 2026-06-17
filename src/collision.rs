use bevy::prelude::*;

use crate::camera::CameraController;
use crate::state::GameState;

#[derive(Component)]
pub struct Collider {
    pub half_extents: Vec3,
}

#[derive(Resource)]
struct LogTimer(Timer);

pub struct CollisionPlugin;

const PLAYER_RADIUS: f32 = 0.4;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(
                Update,
                resolve_collisions
                    .after(crate::camera::camera_controller)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn resolve_collisions(
    mut player: Query<&mut Transform, With<CameraController>>,
    walls: Query<(&Transform, &Collider), Without<CameraController>>,
    time: Res<Time>,
    mut log_timer: ResMut<LogTimer>,
) {
    log_timer.0.tick(time.delta());
    let mut transform = player.single_mut().unwrap();
    let mut player_pos = transform.translation;

    if log_timer.0.just_finished() {
        info!("player position {player_pos:?}");
        info!("wall count: {}", walls.iter().count());
    }
    // AABB math happens in here
    // AABB -- Axis Aligned Bounding Box
    for (wall_transform, collider) in &walls {
        let half = collider.half_extents + Vec3::splat(PLAYER_RADIUS);
        let center = wall_transform.translation;
        let min = center - half;
        let max = center + half;

        // compare player position to min and max x, y, z values to get boolean inside value
        let inside = player_pos.x > min.x
            && player_pos.x < max.x
            && player_pos.y > min.y
            && player_pos.y < max.y
            && player_pos.z > min.z
            && player_pos.z < max.z;

        if !inside {
            continue;
        };

        // calcluate penetration depth
        let p_min = player_pos - min;
        let p_max = max - player_pos;

        // find minimum element
        let i = p_min.min_position();
        let j = p_max.min_position();
        // which corner has min penetration
        let correction = if p_min[i] < p_max[j] {
            (i, -p_min[i])
        } else {
            (j, p_max[j])
        };
        // which axis has min penetration
        let mut pen_vec = Vec3::ZERO;
        pen_vec[correction.0] = correction.1;

        // adjust translation along correct axis
        player_pos += pen_vec;

        // slow timer for printing logs
        if log_timer.0.just_finished() {
            info!("wall center={:.3}, min={:.3}, max={:.3}", center, min, max);
            info!("inside = {}, p_max {}, p_min {}", inside, p_max, p_min);
            info!("pen vec: {}", pen_vec);
        }
    }
    transform.translation = player_pos;
}
