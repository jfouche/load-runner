// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod cursor;
mod debug;
mod in_game;
mod menu;
mod schedule;
mod splash;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(RapierConfiguration::new(100.0))
        .add_plugins(debug::plugin)
        .add_plugins((
            schedule::schedule_plugin,
            ui::UiPlugins,
            splash::splash_plugin,
            menu::menu_plugin,
            in_game::InGamePlugins,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
