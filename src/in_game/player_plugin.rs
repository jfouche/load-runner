use crate::{
    asset_tracking::LoadResource,
    components::{
        character::{
            ground_sensor, AnimationTimer, Climber, Damage, Dying, GroundDetection, GroundSensor,
            InWater, JumpSpeed, Jumping, Life, Movement, Speed,
        },
        enemy::Enemy,
        item::{Item, Items},
        level::{Destructible, COLLISIONS_LAYER},
        player::{DigEvent, Player, PlayerAssets, PlayerDeathEvent},
        GROUP_ENEMY,
    },
    schedule::InGameSet,
    utils::{
        collisions::{start_event_filter, QueryEither},
        invulnerable::Invulnerable,
        iter_ext::IterExt,
    },
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::translation_to_grid_coords};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub fn player_plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>()
        .add_systems(Update, (movement, dig_hole).in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            (
                (animate_walk, animate_jump).after(tick_and_update_sprite),
                animate_death,
            )
                .in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (enemy_hit_player, player_hits_enemy).in_set(InGameSet::CollisionDetection),
        )
        .add_observer(init_player_sprite)
        .add_observer(spawn_ground_sensor)
        .add_observer(player_dying);
}

fn init_player_sprite(
    trigger: Trigger<OnAdd, Player>,
    mut players: Query<&mut Sprite, With<Player>>,
    assets: Res<PlayerAssets>,
) {
    if let Ok(mut sprite) = players.get_mut(trigger.target()) {
        sprite.texture_atlas = Some(TextureAtlas {
            layout: assets.walk_atlas_layout.clone(),
            index: 16,
        });
    }
}

/// Spawn a [Sensor] at the bottom of a collider to detect when it is on the ground
fn spawn_ground_sensor(trigger: Trigger<OnAdd, GroundDetection>, mut commands: Commands) {
    commands.spawn(ground_sensor(trigger.target(), Vec2::new(7.0, 8.0)));
}

fn next_sprite_index_option(indices: &[usize], current: usize) -> Option<usize> {
    let i = indices.iter().position(|&v| v == current)?;
    Some((i + 1) % indices.len())
}

fn next_sprite_index_repeat(indices: &[usize], current: usize) -> usize {
    let idx = next_sprite_index_option(indices, current).unwrap_or(0);
    indices[idx]
}

fn next_sprite_index_once(indices: &[usize], current: usize) -> usize {
    next_sprite_index_option(indices, current)
        .map(|i| indices[i])
        .unwrap_or(current)
}

fn tick_and_update_sprite(
    mut players: Query<
        (&Jumping, &mut Sprite, &mut AnimationTimer),
        (With<Player>, Without<Dying>),
    >,
    assets: Res<PlayerAssets>,
    time: Res<Time>,
) {
    if let Ok((&jumping, mut sprite, mut timer)) = players.single_mut() {
        timer.tick(time.delta());
        if *jumping {
            sprite.image = assets.jump_sprites.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: assets.jump_atlas_layout.clone(),
                index: 0,
            });
        } else {
            sprite.image = assets.walk_sprites.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: assets.walk_atlas_layout.clone(),
                index: 0,
            });
        }
    }
}

fn animate_walk(
    mut players: Query<
        (&Velocity, &Climber, &Jumping, &AnimationTimer, &mut Sprite),
        (With<Player>, Without<Dying>),
    >,
) {
    const MOVE_RIGHT_INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    const MOVE_LEFT_INDICES: [usize; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
    const CLIMB_INDICES: [usize; 8] = [24, 25, 26, 27, 28, 29, 30, 31];
    const FIXED_INDICE: usize = 16;

    if let Ok((velocity, climber, &jumping, timer, mut sprite)) = players.single_mut() {
        if !*jumping && timer.just_finished() {
            let atlas = sprite
                .texture_atlas
                .as_mut()
                .expect("A player should have a TextureAtlas");
            if velocity.is_moving_right() {
                atlas.index = next_sprite_index_repeat(&MOVE_RIGHT_INDICES, atlas.index)
            } else if velocity.is_moving_left() {
                atlas.index = next_sprite_index_repeat(&MOVE_LEFT_INDICES, atlas.index);
            } else if climber.climbing {
                // Climbing
                let idx = if velocity.is_moving_vertical() {
                    // Moving
                    next_sprite_index_repeat(&CLIMB_INDICES, atlas.index)
                } else {
                    // Doesn't move during climbing
                    CLIMB_INDICES[0]
                };
                atlas.index = idx;
            } else {
                // Doesn't move
                atlas.index = FIXED_INDICE;
            }
        }
    }
}

fn animate_jump(
    mut players: Query<
        (&Velocity, &Jumping, &AnimationTimer, &mut Sprite),
        (With<Player>, Without<Dying>),
    >,
) {
    const JUMP_RIGHT_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const JUMP_LEFT_INDICES: [usize; 6] = [6, 7, 8, 9, 10, 11];
    const JUMP_FRONT_INDICES: [usize; 6] = [12, 13, 14, 15, 16, 17];

    if let Ok((velocity, &jumping, timer, mut sprite)) = players.single_mut() {
        if *jumping && timer.just_finished() {
            let atlas = sprite
                .texture_atlas
                .as_mut()
                .expect("A player should have a TextureAtlas");
            if velocity.is_moving_right() {
                atlas.index = next_sprite_index_once(&JUMP_RIGHT_INDICES, atlas.index)
            } else if velocity.is_moving_left() {
                atlas.index = next_sprite_index_once(&JUMP_LEFT_INDICES, atlas.index);
            } else {
                // Doesn't move
                atlas.index = next_sprite_index_once(&JUMP_FRONT_INDICES, atlas.index);
            }
        }
    }
}

fn player_dying(
    trigger: Trigger<OnAdd, Dying>,
    mut players: Query<&mut Sprite, With<Player>>,
    assets: Res<PlayerAssets>,
) {
    if let Ok(mut sprite) = players.get_mut(trigger.target()) {
        sprite.image = assets.death_sprites.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: assets.death_atlas_layout.clone(),
            index: 0,
        });
    }
}

fn animate_death(
    mut commands: Commands,
    time: Res<Time>,
    mut players: Query<(Entity, &mut AnimationTimer, &mut Sprite), (With<Player>, With<Dying>)>,
) {
    const DEATH_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];

    if let Ok((player_entity, mut timer, mut sprite)) = players.single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let atlas = sprite
                .texture_atlas
                .as_mut()
                .expect("A player should have a TextureAtlas");
            match DEATH_INDICES.iter().position(|&v| v == atlas.index) {
                Some(idx) if idx >= DEATH_INDICES.len() - 1 => {
                    commands
                        .entity(player_entity)
                        .remove::<(Dying, AnimationTimer)>();
                    commands.trigger(PlayerDeathEvent);
                }
                Some(idx) => atlas.index = DEATH_INDICES[idx + 1],
                None => unreachable!(),
            }
        }
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut Climber,
            &mut Jumping,
            &GroundDetection,
            &Speed,
            &JumpSpeed,
            &Items,
            &InWater,
        ),
        With<Player>,
    >,
) {
    const BOOTS_JUMP_BONUS: f32 = 1.55;
    const WATER_PENALTY: f32 = 0.4;
    // const SMALL_JUMP_SPEED: f32 = 160.;
    // const BIG_JUMP_SPEED: f32 = 280.;

    for (
        mut velocity,
        mut climber,
        mut jumping,
        ground_detection,
        &speed,
        &jump_speed,
        items,
        &in_water,
    ) in &mut query
    {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };
        let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * *speed;
        if *in_water {
            velocity.linvel.x *= WATER_PENALTY;
        }

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if *in_water {
            velocity.linvel.y = (up - down) * *speed * WATER_PENALTY;
        }

        if climber.climbing {
            velocity.linvel.y = (up - down) * *speed;
        }

        // Jump
        if input.just_pressed(KeyCode::Space)
            && !jumping.0
            && (ground_detection.on_ground || climber.climbing || *in_water)
        {
            jumping.0 = true;
            velocity.linvel.y = *jump_speed;
            if items.contains(Item::Boots) {
                velocity.linvel.y *= BOOTS_JUMP_BONUS;
            }
            climber.climbing = false;
        }
    }
}

fn dig_hole(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    players: Query<&Transform, With<Player>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    levels: Query<(&Transform, &LevelIid), Without<Player>>,
    diggable_cells: Query<(Entity, &GridCoords, &ChildOf), With<Destructible>>,
    layers: Query<(Entity, &LayerMetadata)>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) -> Result {
    let dig_left = input.just_pressed(KeyCode::KeyQ);
    let dig_right = input.just_pressed(KeyCode::KeyE);

    if !dig_left && !dig_right {
        // Player don't dig
        return Ok(());
    }

    let player_transform = players.single()?;
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .ok_or("Project should exist")?;

    let (level_transform, layer_info) = levels
        .iter()
        .filter_map(|(transform, iid)| {
            let level = ldtk_project.get_raw_level_by_iid(&iid.to_string())?;
            let layer_info = level.layer_instances.as_ref()?.get(COLLISIONS_LAYER)?;
            level_selection
                .is_match(&LevelIndices::default(), level)
                .then_some((transform, layer_info))
        })
        .single()?;

    // get 'Collisions' layer entities where tiles are on
    let layers_entity = layers
        .iter()
        .filter_map(|(layer_entity, metadata)| {
            (metadata.iid == layer_info.iid).then_some(layer_entity)
        })
        .collect::<Vec<_>>();

    // get player coords
    let translation = player_transform.translation.xy() - level_transform.translation.xy();
    let player_coord = translation_to_grid_coords(translation, IVec2::splat(layer_info.grid_size));

    // Get the dig coords
    let x = if dig_left {
        player_coord.x - 1
    } else {
        player_coord.x + 1
    };
    let cell_coord = GridCoords {
        x,
        y: player_coord.y - 1,
    };

    // get the digged cell
    if let Ok(cell_entity) = diggable_cells
        .iter()
        .filter(|(_, &coord, ChildOf(le))| layers_entity.contains(le) && coord == cell_coord)
        .map(|(e, _c, _)| e)
        .single()
    {
        commands.trigger_targets(DigEvent, cell_entity);
    }
    Ok(())
}

fn enemy_hit_player(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Life), With<Player>>,
    enemies: Query<&Damage, With<Enemy>>,
) -> Result {
    let (player_entity, mut life) = players.single_mut()?;
    if let Ok(damage) = collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| enemies.get_either(e1, e2))
        .filter(|(_damage, _enemy_entity, other_entity)| *other_entity == player_entity)
        .map(|(damage, _enemy_entity, _player_entity)| damage)
        .single()
    {
        life.hit(damage.0);
        if life.is_dead() {
            commands.entity(player_entity).insert(Dying);
        } else {
            // Make player invulnerable
            commands
                .entity(player_entity)
                .insert(Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY));
        }
    }
    Ok(())
}

fn player_hits_enemy(
    mut commands: Commands,
    ground_detectors: Query<(), With<Player>>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
    mut enemies: Query<(Entity, &mut Life), With<Enemy>>,
) {
    for sensor in &ground_sensors {
        if ground_detectors.get(sensor.ground_detection_entity).is_ok() {
            sensor.intersecting_ground_entities.iter().for_each(|e| {
                if let Ok((entity, mut life)) = enemies.get_mut(*e) {
                    life.hit(1);
                    if life.is_dead() {
                        // TODO: do not kill it like this
                        commands.entity(entity).despawn();
                    }
                }
            });
        }
    }
}
