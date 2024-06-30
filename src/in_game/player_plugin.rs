use std::time::Duration;

use crate::in_game::collisions::*;
use crate::utils::*;
use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::GROUP_ENEMY;

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerDiedEvent>()
        .add_systems(Update, spawn_ground_sensor)
        .add_systems(Update, movement.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            player_hits_enemy.in_set(InGameSet::CollisionDetection),
        );
}

/// Spawn a [Sensor] at the bottom of a collider to detect when it is on the ground
fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, collider) in &detect_ground_for {
        if let Some(cuboid) = collider.as_cuboid() {
            info!("spawn_ground_sensor for {entity:?}");
            commands.entity(entity).with_children(|builder| {
                builder.spawn(GroundSensorCollider::new(entity, cuboid.half_extents()));
            });
        }
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
) {
    const MOVE_SPEED: f32 = 200.;
    const JUMP_SPEED: f32 = 400.;

    for (mut velocity, mut climber, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * MOVE_SPEED;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * MOVE_SPEED;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = JUMP_SPEED;
            climber.climbing = false;
        }
    }
}

fn player_hits_enemy(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Life), With<Player>>,
    enemies: Query<&Damage, With<Enemy>>,
    mut died_events: EventWriter<PlayerDiedEvent>,
) {
    if let Ok((player_entity, mut life)) = players.get_single_mut() {
        if let Some(damage) = collisions
            .read()
            .filter_map(start_event_filter)
            .filter_map(|(&e1, &e2)| enemies.get_either(e1, e2))
            .filter(|(_damage, _enemy_entity, other_entity)| *other_entity == player_entity)
            .map(|(damage, _enemy_entity, _player_entity)| damage)
            .next()
        {
            life.hit(damage.0);
            if life.is_dead() {
                died_events.send(PlayerDiedEvent);
            } else {
                // Make player invulnerable
                commands.entity(player_entity).insert((
                    Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY),
                    Blink::new(Duration::from_secs_f32(0.15)),
                ));
            }
        }
    }
}
