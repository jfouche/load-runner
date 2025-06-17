use crate::{
    components::{
        character::{
            Climber, GroundDetection, GroundSensor, InWater, JumpSpeed, Jumping, Life, Speed,
        },
        level::{Climbable, LdtkWaterCell, COLLISIONS_LAYER},
    },
    schedule::InGameSet,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::translation_to_grid_coords};
use bevy_rapier2d::prelude::*;

pub fn character_plugin(app: &mut App) {
    app.register_type::<Life>()
        .register_type::<Speed>()
        .register_type::<InWater>()
        .register_type::<Jumping>()
        .register_type::<Climber>()
        .register_type::<JumpSpeed>()
        .add_systems(
            Update,
            (
                update_on_ground,
                update_in_water,
                update_jumping,
                ignore_gravity_if_climbing,
            )
                .in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (detect_climb_range, ground_detection).in_set(InGameSet::CollisionDetection),
        );
}

fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*e1), climbables.get(*e2))
                {
                    climber.intersecting_climbables.insert(climbable);
                }
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*e2), climbables.get(*e1))
                {
                    climber.intersecting_climbables.insert(climbable);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*e1), climbables.get(*e2))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }

                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*e2), climbables.get(*e1))
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

fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();

            info!("update_on_ground : {}", ground_detection.on_ground);
        }
    }
}

fn update_in_water(
    mut in_waters: Query<(&Transform, &mut InWater)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_query: Query<(Entity, &Transform, &LevelIid)>,
    water_cells: Query<(&GridCoords, &ChildOf), With<LdtkWaterCell>>,
    parents: Query<&ChildOf, Without<LdtkWaterCell>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    let Ok(ldtk_project) = ldtk_projects.single() else {
        return;
    };
    for (character_transform, mut in_water) in &mut in_waters {
        level_query
            .iter()
            .filter_map(|(entity, transform, iid)| {
                let ldtk_project = ldtk_project_assets.get(ldtk_project)?;
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

                in_water.0 = water_cells.iter().any(|(&coord, &ChildOf(parent))| {
                    if coord == character_coord {
                        if let Ok(&ChildOf(grandparent)) = parents.get(parent) {
                            if grandparent == level_entity {
                                return true;
                            }
                        }
                    }
                    false
                });
            });
    }
}

fn update_jumping(
    mut characters: Query<(&mut Jumping, &GroundDetection), Changed<GroundDetection>>,
) {
    for (mut jumping, ground_detection) in &mut characters {
        if ground_detection.on_ground {
            jumping.0 = false;
        }
    }
}
