use crate::{
    components::despawn_all,
    schedule::{GameState, InGameState},
    theme::widget,
};
use bevy::prelude::*;

pub fn pause_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Pause), spawn_pause_menu)
        .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
        // .add_systems(Update, back_to_game.run_if(in_state(InGameState::Pause)))
        ;
}

#[derive(Component)]
struct PauseMenu;

fn pause_menu() -> impl Bundle {
    (
        PauseMenu,
        Name::new("PauseMenu"),
        widget::popup(),
        children![
            widget::popup_title("Pause"),
            widget::menu_button("Back to game", on_back_to_game),
            widget::menu_button("Quit game", on_quit_game)
        ],
    )
}
fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(pause_menu());
}

fn on_back_to_game(_trigger: Trigger<Pointer<Click>>, mut state: ResMut<NextState<InGameState>>) {
    state.set(InGameState::Running);
}

fn on_quit_game(_trigger: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
