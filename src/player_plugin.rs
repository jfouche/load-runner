use std::collections::HashSet;

use crate::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Update, spawn_ground_sensor)
        .add_systems(Update, (dbg_player_items, movement));
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

fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            commands.entity(entity).with_children(|builder| {
                let Vec2 {
                    x: half_extents_x,
                    y: half_extents_y,
                } = cuboid.half_extents();

                builder.spawn((
                    ActiveEvents::COLLISION_EVENTS,
                    Collider::cuboid(half_extents_x / 2.0, 2.),
                    Sensor,
                    Transform::from_translation(Vec3::new(0., -half_extents_y, 0.)),
                    GlobalTransform::default(),
                    GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    },
                ));
            });
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
