use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

#[derive(Component)]
struct MenuUi;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), lock_cursor)
            .add_systems(OnEnter(GameState::Paused), release_cursor)
            .add_systems(OnEnter(GameState::Menu), (setup_menu, release_cursor))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(Update, toggle_cursor_grab)
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)));
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Text::new("Press space to start"),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        MenuUi,
    ));
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(GameState::Playing);
    }
}

fn set_cursor(cursor: &mut CursorOptions, locked: bool) {
    cursor.grab_mode = if locked {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    cursor.visible = !locked;
}

fn lock_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    set_cursor(&mut cursor.single_mut().unwrap(), true);
}

fn release_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    set_cursor(&mut cursor.single_mut().unwrap(), false);
}

fn toggle_cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut cursor = cursor.single_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        next_game_state.set(GameState::Paused);
    }
    if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        next_game_state.set(GameState::Playing);
    }
}
