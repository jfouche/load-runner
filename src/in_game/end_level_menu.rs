use crate::{
    schedule::{GameState, InGameState},
    theme::widget,
};
use bevy::prelude::*;

pub fn end_level_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::PlayerEndedLevel), spawn_menu);
}

#[derive(Component)]
struct EndLevelMenu;

fn end_level_menu() -> impl Bundle {
    (
        EndLevelMenu,
        Name::new("EndLevelMenu"),
        widget::centered(),
        children![widget::menu_button("Quit game", on_quit_game)],
    )
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((end_level_menu(), StateScoped(InGameState::PlayerEndedLevel)));
}

fn on_quit_game(
    _trigger: Trigger<Pointer<Click>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_in_game_state.set(InGameState::Disabled);
    next_game_state.set(GameState::Menu);
}
