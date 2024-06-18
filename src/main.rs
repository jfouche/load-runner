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
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration::new(100.0))
        .add_plugins((
            level_plugin::level_plugin,
            player_plugin::player_plugin,
            enemy_plugin::enemy_plugin,
            character_plugin::character_plugin,
        ))
        .add_systems(Update, debug_collisions)
        .run();
}

fn debug_collisions(mut collisions: EventReader<CollisionEvent>) {
    for collision in collisions.read() {
        info!("Collision event: {collision:?}");
    }
}
