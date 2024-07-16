use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::translation_to_grid_coords};
use bevy_rapier2d::prelude::*;

pub fn character_plugin(app: &mut App) {
    app.register_type::<Life>()
        .register_type::<Speed>()
        .register_type::<InWater>()
        .add_systems(
            Update,
            (
                detect_climb_range,
                ignore_gravity_if_climbing,
                ground_detection,
            )
                .in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (update_on_ground, update_in_water).in_set(InGameSet::EntityUpdate),
        );
}

fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.insert(climbable);
                }
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.insert(climbable);
                };
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }

                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }
            }
        }
    }
}

fn ignore_gravity_if_climbing(mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}

fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

fn update_in_water(
    mut in_waters: Query<(&Transform, &mut InWater)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_query: Query<(Entity, &Transform, &LevelIid), Without<OrthographicProjection>>,
    water_cells: Query<(&GridCoords, &Parent), With<Water>>,
    parents: Query<&Parent, Without<Water>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    for (character_transform, mut in_water) in &mut in_waters {
        level_query
            .iter()
            .filter_map(|(entity, transform, iid)| {
                let ldtk_project = ldtk_project_assets.get(ldtk_projects.single())?;
                let level = ldtk_project.get_raw_level_by_iid(&iid.to_string())?;
                let layer_info = level.layer_instances.as_ref()?.get(COLLISIONS_LAYER)?;
                level_selection
                    .is_match(&LevelIndices::default(), level)
                    .then_some((entity, transform, layer_info))
            })
            .for_each(|(level_entity, level_transform, layer_info)| {
                let translation =
                    character_transform.translation.xy() - level_transform.translation.xy();
                let character_coord =
                    translation_to_grid_coords(translation, IVec2::splat(layer_info.grid_size));
                warn!("update_in_water: coord: {character_coord:?}");

                in_water.0 = water_cells.iter().any(|(&coord, parent)| {
                    if coord == character_coord {
                        if let Ok(grandparent) = parents.get(parent.get()) {
                            if grandparent.get() == level_entity {
                                warn!("update_in_water: coord: {coord:?}, level: {level_entity:?}");
                                return true;
                            }
                        }
                    }
                    false
                });
            });
    }
}
