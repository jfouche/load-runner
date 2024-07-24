use super::GROUP_ENEMY;
use crate::in_game::collisions::*;
use crate::utils::*;
use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerDeathEvent>()
        // STARTUP
        .add_systems(Startup, load_assets)
        .add_systems(Update, (spawn_ground_sensor, set_texture_atlas))
        // IN GAME
        .add_systems(Update, movement.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            (
                (animate_walk, animate_jump).after(tick_and_update_sprite),
                player_dying,
                animate_death,
            )
                .in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (enemy_hit_player, player_hits_enemy).in_set(InGameSet::CollisionDetection),
        );
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // WALK
    let walk_sprites = asset_server.load("player/walk.png");
    let walk_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        8,
        4,
        Some(UVec2::splat(64)),
        Some(UVec2::splat(32)),
    );
    let walk_atlas_layout = texture_atlas_layouts.add(walk_layout);

    // JUMP
    let jump_sprites = asset_server.load("player/jump.png");
    let jump_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        6,
        4,
        Some(UVec2::splat(64)),
        Some(UVec2::splat(32)),
    );
    let jump_atlas_layout = texture_atlas_layouts.add(jump_layout);

    // DEATH
    let death_sprites = asset_server.load("player/death.png");
    let death_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        6,
        4,
        Some(UVec2::splat(64)),
        Some(UVec2::splat(32)),
    );
    let death_atlas_layout = texture_atlas_layouts.add(death_layout);

    // ASSET RESOURCE
    let assets = PlayerAssets {
        walk_sprites,
        walk_atlas_layout,
        jump_sprites,
        jump_atlas_layout,
        death_sprites,
        death_atlas_layout,
    };
    commands.insert_resource(assets);
}

fn set_texture_atlas(
    mut players: Query<&mut TextureAtlas, Added<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok(mut atlas) = players.get_single_mut() {
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            8,
            4,
            Some(UVec2::splat(64)),
            Some(UVec2::splat(32)),
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        atlas.layout = texture_atlas_layout;
        atlas.index = 16;
    }
}

/// Spawn a [Sensor] at the bottom of a collider to detect when it is on the ground
fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, _collider) in &detect_ground_for {
        commands.entity(entity).with_children(|builder| {
            builder.spawn(GroundSensorCollider::new(entity, Vec2::new(7.0, 8.0)));
        });
    }
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
        (
            &Jumping,
            &mut Handle<Image>,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        (With<Player>, Without<Dying>),
    >,
    assets: Res<PlayerAssets>,
    time: Res<Time>,
) {
    if let Ok((&jumping, mut image, mut timer, mut atlas)) = players.get_single_mut() {
        timer.tick(time.delta());
        if *jumping {
            *image = assets.jump_sprites.clone();
            atlas.layout = assets.jump_atlas_layout.clone();
        } else {
            *image = assets.walk_sprites.clone();
            atlas.layout = assets.walk_atlas_layout.clone();
        }
    }
}

fn animate_walk(
    mut players: Query<
        (
            &Velocity,
            &Climber,
            &Jumping,
            &AnimationTimer,
            &mut TextureAtlas,
        ),
        (With<Player>, Without<Dying>),
    >,
) {
    const MOVE_RIGHT_INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    const MOVE_LEFT_INDICES: [usize; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
    const CLIMB_INDICES: [usize; 8] = [24, 25, 26, 27, 28, 29, 30, 31];
    const FIXED_INDICE: usize = 16;

    if let Ok((velocity, climber, &jumping, timer, mut atlas)) = players.get_single_mut() {
        if !*jumping && timer.just_finished() {
            if velocity.move_right() {
                atlas.index = next_sprite_index_repeat(&MOVE_RIGHT_INDICES, atlas.index)
            } else if velocity.move_left() {
                atlas.index = next_sprite_index_repeat(&MOVE_LEFT_INDICES, atlas.index);
            } else if climber.climbing {
                // Climbing
                let idx = if velocity.climb() {
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
        (&Velocity, &Jumping, &AnimationTimer, &mut TextureAtlas),
        (With<Player>, Without<Dying>),
    >,
) {
    const JUMP_RIGHT_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const JUMP_LEFT_INDICES: [usize; 6] = [6, 7, 8, 9, 10, 11];
    const JUMP_FRONT_INDICES: [usize; 6] = [12, 13, 14, 15, 16, 17];

    if let Ok((velocity, &jumping, timer, mut atlas)) = players.get_single_mut() {
        if *jumping && timer.just_finished() {
            if velocity.move_right() {
                atlas.index = next_sprite_index_once(&JUMP_RIGHT_INDICES, atlas.index)
            } else if velocity.move_left() {
                atlas.index = next_sprite_index_once(&JUMP_LEFT_INDICES, atlas.index);
            } else {
                // Doesn't move
                atlas.index = next_sprite_index_once(&JUMP_FRONT_INDICES, atlas.index);
            }
        }
    }
}

fn player_dying(
    mut players: Query<(&mut TextureAtlas, &mut Handle<Image>), (With<Player>, Added<Dying>)>,
    assets: Res<PlayerAssets>,
) {
    if let Ok((mut atlas, mut image)) = players.get_single_mut() {
        *image = assets.death_sprites.clone();
        atlas.layout = assets.death_atlas_layout.clone();
        atlas.index = 0;
    }
}

fn animate_death(
    mut commands: Commands,
    time: Res<Time>,
    mut players: Query<
        (Entity, &mut AnimationTimer, &mut TextureAtlas),
        (With<Player>, With<Dying>),
    >,
    mut death_events: EventWriter<PlayerDeathEvent>,
) {
    const DEATH_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];

    if let Ok((player_entity, mut timer, mut atlas)) = players.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            match DEATH_INDICES.iter().position(|&v| v == atlas.index) {
                Some(idx) if idx >= DEATH_INDICES.len() - 1 => {
                    death_events.send(PlayerDeathEvent);
                    commands
                        .entity(player_entity)
                        .remove::<(Dying, AnimationTimer, ColliderBundle)>();
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

fn enemy_hit_player(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Life), With<Player>>,
    enemies: Query<&Damage, With<Enemy>>,
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
                commands.entity(player_entity).insert(Dying);
            } else {
                // Make player invulnerable
                commands
                    .entity(player_entity)
                    .insert(Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY));
            }
        }
    }
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
                        commands.entity(entity).despawn_recursive();
                    }
                }
            });
        }
    }
}
