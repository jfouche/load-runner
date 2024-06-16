// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod character_plugin;
mod components;
mod enemy_plugin;
mod level_plugin;
mod player_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),))
        .insert_resource(RapierConfiguration::new(100.0))
        .add_plugins((
            level_plugin::level_plugin,
            player_plugin::player_plugin,
            enemy_plugin::enemy_plugin,
            character_plugin::character_plugin,
        ))
        .run();
}
