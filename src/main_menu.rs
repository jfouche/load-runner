use crate::{
    components::{despawn_all, GameState},
    ui::*,
};
use bevy::{app::AppExit, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>)
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::Menu)),
        );
}

#[derive(Component)]
struct MainMenu;

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    // Settings,
    // SettingsSound,
    // SettingsDisplay,
    // BackToMainMenu,
    // BackToSettings,
    ExitApplication,
    // QuitGame,
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::GRAY));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((MainMenu, centered())).with_children(|wnd| {
        wnd.spawn(menu()).with_children(|menu| {
            menu.spawn(menu_title("Load-Runner"));
            menu.spawn((button_bundle(), MenuButtonAction::PlayGame))
                .with_children(|parent| {
                    parent.spawn(button_text("New Game"));
                });
            menu.spawn((button_bundle(), MenuButtonAction::ExitApplication))
                .with_children(|parent| {
                    parent.spawn(button_text("Exit"));
                });
        });
    });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::ExitApplication => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::PlayGame => {
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
