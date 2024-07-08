use crate::{components::*, cursor::*, schedule::InGameSet, utils::*};
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier2d::prelude::*;

mod character_plugin;
mod collisions;
mod death_menu;
mod end_level_menu;
mod enemy_plugin;
mod hud_plugin;
mod item_plugin;
mod level_plugin;
mod pause_menu;
mod player_plugin;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;

pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(death_menu::plugin)
            .add(character_plugin::character_plugin)
            .add(enemy_plugin::enemy_plugin)
            .add(hud_plugin::hud_plugin)
            .add(level_plugin::level_plugin)
            .add(player_plugin::player_plugin)
            .add(pause_menu::pause_menu_plugin)
            .add(item_plugin::item_plugin)
            .add(end_level_menu::end_level_menu_plugin)
            .add(in_game_plugin)
    }
}

fn in_game_plugin(app: &mut App) {
    app.add_systems(Startup, stop_physics)
        .add_systems(OnEnter(GameState::InGame), (set_background, grab_cursor))
        .add_systems(OnExit(GameState::InGame), (ungrab_cursor, reset_physics))
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
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

fn pause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
    for mut blink in &mut blinks {
        blink.pause(true);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(true);
    }
}

fn unpause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
    for mut blink in &mut blinks {
        blink.pause(false);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(false);
    }
}

fn start_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = true;
    physics.query_pipeline_active = true;
}

fn stop_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = false;
    physics.query_pipeline_active = false;
}

fn reset_physics(mut commands: Commands) {
    commands.insert_resource(Events::<CollisionEvent>::default());
    commands.insert_resource(Events::<ContactForceEvent>::default());
}
