use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));
    let floor_mat = materials.add(Color::srgb(0.3, 0.5, 0.3));

    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_mat)
    ));
}