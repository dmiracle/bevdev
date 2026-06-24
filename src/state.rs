use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum Pause {
    #[default]
    Running,
    Paused,
}

#[derive(Component)]
struct MenuUi;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<Pause>()
            .add_systems(OnEnter(Pause::Running), lock_cursor)
            .add_systems(OnEnter(Pause::Paused), release_cursor)
            .add_systems(OnEnter(GameState::Menu), (setup_menu, release_cursor))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(Update, pause_game.run_if(in_state(Pause::Running)))
            .add_systems(Update, resume_game.run_if(in_state(Pause::Paused)))
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(Update, quit_to_menu.run_if(in_state(Pause::Paused)));
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

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<GameState>>,
    mut sub_next: ResMut<NextState<Pause>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next.set(GameState::InGame);
        sub_next.set(Pause::Running);
    }
}

fn pause_game(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<Pause>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(Pause::Paused);
    }
}

fn resume_game(buttons: Res<ButtonInput<MouseButton>>, mut next: ResMut<NextState<Pause>>) {
    if buttons.just_pressed(MouseButton::Left) {
        next.set(Pause::Running);
    }
}

fn quit_to_menu(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        next.set(GameState::Menu);
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
