// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

mod asset_tracking;
mod camera;
mod components;
mod cursor;
mod debug;
mod in_game;
mod main_menu;
mod schedule;
mod splash;
mod theme;
mod ui;
mod utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .add_plugins(debug::plugin)
        .add_plugins((
            ui::progressbar::plugin,
            ui::fade::plugin,
            utils::blink::BlinkPlugin,
            utils::invulnerable::InvulnerabilityPlugin,
            utils::despawn_after::despawn_after_plugin,
            theme::theme_plugin,
        ))
        .add_plugins((
            asset_tracking::asset_tracking_plugin,
            schedule::schedule_plugin,
            camera::camera_plugin,
            splash::splash_plugin,
            main_menu::main_menu_plugin,
            in_game::InGamePlugins,
        ))
        .run();
}
