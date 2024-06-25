use crate::{components::*, cursor::*, schedule::InGameSet};
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier2d::prelude::*;

mod character_plugin;
mod enemy_plugin;
mod hud_plugin;
mod item_plugin;
mod level_plugin;
mod pause_menu;
mod player_plugin;

pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(character_plugin::character_plugin)
            .add(enemy_plugin::enemy_plugin)
            .add(hud_plugin::hud_plugin)
            .add(level_plugin::level_plugin)
            .add(player_plugin::player_plugin)
            .add(pause_menu::pause_menu_plugin)
            .add(in_game_plugin)
    }
}

fn in_game_plugin(app: &mut App) {
    app.add_systems(Startup, stop_physics)
        .add_systems(OnEnter(GameState::InGame), (set_background, grab_cursor))
        .add_systems(OnExit(GameState::InGame), ungrab_cursor)
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(Update, switch_to_pause.in_set(InGameSet::UserInput));
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::BLACK));
}

fn switch_to_pause(mut state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(InGameState::Pause);
    }
}

fn start_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = true;
}

fn stop_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = false;
}
