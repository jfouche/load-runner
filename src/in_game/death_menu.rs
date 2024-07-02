use super::*;
use crate::schedule::InGameSet;
use crate::ui::*;

#[derive(Component)]
struct DeathMenu;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_player_death.in_set(InGameSet::EntityUpdate))
        .add_systems(
            OnEnter(InGameState::PlayerDied),
            (ungrab_cursor, spawn_death_menu),
        )
        .add_systems(OnExit(InGameState::PlayerDied), despawn_all::<DeathMenu>)
        .add_systems(
            Update,
            menu_action.run_if(in_state(InGameState::PlayerDied)),
        );
}

#[derive(Component, PartialEq)]
enum MenuButtonAction {
    QuitGame,
}

fn spawn_death_menu(mut commands: Commands) {
    commands
        .spawn((centered(), Name::new("DeathMenu"), DeathMenu))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("You died !"));
                menu.spawn((button_bundle(), MenuButtonAction::QuitGame))
                    .with_children(|parent| {
                        parent.spawn(button_text("Quit"));
                    });
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed && *menu_button_action == MenuButtonAction::QuitGame
        {
            in_game_state.set(InGameState::Disabled);
            game_state.set(GameState::Menu);
        }
    }
}

fn on_player_death(
    mut death_events: EventReader<PlayerDeathEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for _ in death_events.read() {
        in_game_state.set(InGameState::PlayerDied);
    }
}
