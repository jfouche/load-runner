use crate::{
    components::{despawn_all, GameState},
    ui::*,
};
use bevy::prelude::*;

use super::InGameState;

pub fn pause_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Pause), spawn_menu)
        .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(InGameState::Pause)),
        );
}

#[derive(Component)]
struct PauseMenu;

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    // Settings,
    // SettingsSound,
    // SettingsDisplay,
    // BackToMainMenu,
    // BackToSettings,
    QuitGame,
}

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((PauseMenu, centered()))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("Load-Runner - Pause"));
                menu.spawn((button_bundle(), MenuButtonAction::PlayGame))
                    .with_children(|parent| {
                        parent.spawn(button_text("Play game"));
                    });
                menu.spawn((button_bundle(), MenuButtonAction::QuitGame))
                    .with_children(|parent| {
                        parent.spawn(button_text("Quit game"));
                    });
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitGame => {
                    next_in_game_state.set(InGameState::Disabled);
                    next_game_state.set(GameState::Menu);
                }
                MenuButtonAction::PlayGame => {
                    next_in_game_state.set(InGameState::Running);
                }
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_in_game_state.set(InGameState::Running);
    }
}
