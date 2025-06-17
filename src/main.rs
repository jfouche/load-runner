mod asset_tracking;
mod camera;
mod components;
mod cursor;
mod in_game;
mod main_menu;
mod schedule;
mod splash;
mod theme;
mod ui;
mod utils;

#[cfg(feature = "dev")]
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((
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
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(debug::plugin);

    app.run();
}
