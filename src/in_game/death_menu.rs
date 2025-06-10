use crate::{
    components::{despawn_all, player::PlayerDeathEvent},
    cursor::ungrab_cursor,
    schedule::{GameState, InGameState},
    theme::widget,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InGameState::PlayerDied),
        (ungrab_cursor, spawn_death_menu),
    )
    .add_systems(OnExit(InGameState::PlayerDied), despawn_all::<DeathMenu>)
    .add_systems(
        Update,
        back_to_menu
            .run_if(in_state(InGameState::PlayerDied).and(input_just_pressed(KeyCode::Enter))),
    )
    .add_observer(on_player_death);
}

#[derive(Component)]
struct DeathMenu;

fn death_menu() -> impl Bundle {
    (
        DeathMenu,
        Name::new("DeathMenu"),
        widget::popup(),
        children![
            widget::popup_title("You died !"),
            widget::button("Back to menu", on_back_to_menu)
        ],
    )
}

fn spawn_death_menu(mut commands: Commands) {
    commands.spawn(death_menu());
}

fn on_player_death(
    _trigger: Trigger<PlayerDeathEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    in_game_state.set(InGameState::PlayerDied);
}

fn back_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    game_state.set(GameState::Menu);
    in_game_state.set(InGameState::Disabled);
}

fn on_back_to_menu(
    _trigger: Trigger<Pointer<Click>>,
    game_state: ResMut<NextState<GameState>>,
    in_game_state: ResMut<NextState<InGameState>>,
) {
    back_to_menu(game_state, in_game_state);
}
