use crate::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Update, (dbg_player_items, movement));
}

fn dbg_player_items(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Items, &EntityInstance), With<Player>>,
) {
    for (items, entity_instance) in &mut query {
        if input.just_pressed(KeyCode::KeyP) {
            dbg!(&items);
            dbg!(&entity_instance);
        }
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
) {
    for (mut velocity, mut climber, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.;
            climber.climbing = false;
        }
    }
}
