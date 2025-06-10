use crate::{
    components::despawn_all,
    schedule::{GameState, InGameState},
    theme::widget,
    ui::*,
};
use bevy::prelude::*;

pub fn end_level_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::PlayerEndedLevel), spawn_menu)
        .add_systems(
            OnExit(InGameState::PlayerEndedLevel),
            despawn_all::<EndLevelMenu>,
        );
}

#[derive(Component)]
struct EndLevelMenu;

fn end_level_menu() -> impl Bundle {
    (
        EndLevelMenu,
        Name::new("EndLevelMenu"),
        centered(),
        children![widget::button("Quit game", on_quit_game)],
    )
}

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
enum MenuButtonAction {
    QuitGame,
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn(end_level_menu());
}

fn on_quit_game(
    _trigger: Trigger<Pointer<Click>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_in_game_state.set(InGameState::Disabled);
    next_game_state.set(GameState::Menu);
}
