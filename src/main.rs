// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod cursor;
mod debug;
mod in_game;
mod main_menu;
mod schedule;
mod splash;
mod ui;
mod utils;

const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Load-Runner".into(),
                        position: WindowPosition::At(IVec2::ZERO),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
        ))
        .insert_resource(RapierConfiguration::new(PIXELS_PER_METER))
        .add_plugins(debug::plugin)
        .add_plugins((
            schedule::schedule_plugin,
            utils::UtilsPlugins,
            ui::UiPlugins,
            splash::splash_plugin,
            main_menu::main_menu_plugin,
            in_game::InGamePlugins,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
