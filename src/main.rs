use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Component)]
struct CameraController {
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}

#[derive(Component)]
struct Collider {
    half_extents: Vec3,
}

#[derive(Resource)]
struct LogTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LogTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .add_systems(Startup, (setup, grab_cursor))
        .add_systems(Update, toggle_cursor_grab)
        .add_systems(Update, (camera_controller, resolve_collisions).chain())
        .run();
}

fn resolve_collisions(
    mut player: Query<&mut Transform, With<CameraController>>,
    walls: Query<(&Transform, &Collider), Without<CameraController>>,
    time: Res<Time>,
    mut log_timer: ResMut<LogTimer>,
) {
    log_timer.0.tick(time.delta());
    let player_pos = player.single_mut().unwrap().translation;

    if log_timer.0.just_finished() {
        info!("player position {player_pos:?}");
        info!("wall cournt: {}", walls.iter().count());
    }
    for (wall_transform, collider) in &walls {
        let center = wall_transform.translation;
        let min = center - collider.half_extents;
        let max = center + collider.half_extents;
        if log_timer.0.just_finished() {
            info!("wall center={:.3}, min={:.3}, max={:.3}", center, min, max);
        }
    }
}
fn grab_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    let mut cursor = cursor.single_mut().unwrap();
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn toggle_cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let mut cursor = cursor.single_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
    if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
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
            half_extents: Vec3::new(0.5, 0.5, 0.5)
        },
    ));
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

fn camera_controller(
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
