use crate::{
    components::despawn_all,
    schedule::GameState,
    theme::{palette::MAIN_MENU_BACKGROUND, widget},
};
use bevy::{app::AppExit, color::palettes::css::GRAY, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>);
}

#[derive(Component)]
struct MainMenu;

fn main_menu() -> impl Bundle {
    (
        MainMenu,
        widget::ui_root("MainMenu"),
        BackgroundColor(MAIN_MENU_BACKGROUND),
        GlobalZIndex(2),
        children![
            widget::header("Load-Runner"),
            widget::menu_button("New game", on_new_game),
            widget::menu_button("Exit", on_exit),
        ],
    )
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(GRAY.into()));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn(main_menu());
}

fn on_new_game(
    _trigger: Trigger<Pointer<Click>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::InGame);
}

fn on_exit(_trigger: Trigger<Pointer<Click>>, mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.write(AppExit::Success);
}
