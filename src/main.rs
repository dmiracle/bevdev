mod camera;
mod collision;
mod debug;
mod dungeon;
mod state;
mod world;

use bevy::prelude::*;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use dungeon::DungeonPlugin;
use state::StatePlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            StatePlugin,
            CameraPlugin,
            CollisionPlugin,
            WorldPlugin,
            DebugPlugin,
            DungeonPlugin,
        ))
        .run();
}
