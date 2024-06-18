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

fn debug_collisions(mut collisions: EventReader<CollisionEvent>, names: Query<DebugName>) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {
                let n1 = names.get(*e1).unwrap();
                let n2 = names.get(*e2).unwrap();
                info!("CollisionEvent::Started({n1:?}, {n2:?}, {flag:?})");
            }
            CollisionEvent::Stopped(e1, e2, flag) => {
                let n1 = names.get(*e1).unwrap();
                let n2 = names.get(*e2).unwrap();
                info!("CollisionEvent::Stopped({n1:?}, {n2:?}, {flag:?})");
            }
        }
    }
}
