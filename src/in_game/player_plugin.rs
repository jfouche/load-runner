use std::time::Duration;

use crate::in_game::collisions::*;
use crate::utils::*;
use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::GROUP_ENEMY;

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerDeathEvent>()
        .add_systems(Startup, load_assets)
        .add_systems(Update, (spawn_ground_sensor, set_texture_atlas))
        .add_systems(Update, movement.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            (animate_player, player_dying, animate_death).in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            player_hits_enemy.in_set(InGameSet::CollisionDetection),
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
        Vec2::splat(16.0),
        8,
        4,
        Some(Vec2::splat(64.0)),
        Some(Vec2::splat(32.0)),
    );
    let walk_atlas_layout = texture_atlas_layouts.add(walk_layout);

    // DEATH
    let death_sprites = asset_server.load("player/death.png");
    let death_layout = TextureAtlasLayout::from_grid(
        Vec2::splat(16.0),
        6,
        4,
        Some(Vec2::splat(64.0)),
        Some(Vec2::splat(32.0)),
    );
    let death_atlas_layout = texture_atlas_layouts.add(death_layout);

    // ASSET RESOURCE
    let assets = PlayerAssets {
        walk_sprites,
        walk_atlas_layout,
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
            Vec2::splat(16.0),
            8,
            4,
            Some(Vec2::splat(64.0)),
            Some(Vec2::splat(32.0)),
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
    for (entity, collider) in &detect_ground_for {
        if let Some(cuboid) = collider.as_cuboid() {
            info!("spawn_ground_sensor for {entity:?}");
            commands.entity(entity).with_children(|builder| {
                builder.spawn(GroundSensorCollider::new(entity, cuboid.half_extents()));
            });
        }
    }
}

fn sprite_index(indices: &[usize], current: usize) -> usize {
    match indices.iter().position(|&v| v == current) {
        Some(idx) => (idx + 1) % indices.len(),
        None => 0,
    }
}

fn animate_player(
    time: Res<Time>,
    mut players: Query<
        (&Velocity, &Climber, &mut AnimationTimer, &mut TextureAtlas),
        (With<Player>, Without<Dying>),
    >,
) {
    const MOVE_RIGHT_INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    const MOVE_LEFT_INDICES: [usize; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
    const CLIMB_INDICES: [usize; 8] = [24, 25, 26, 27, 28, 29, 30, 31];

    if let Ok((velocity, climber, mut timer, mut atlas)) = players.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if velocity.linvel.x > f32::EPSILON {
                // Move right
                let idx = sprite_index(&MOVE_RIGHT_INDICES, atlas.index);
                atlas.index = MOVE_RIGHT_INDICES[idx];
            } else if velocity.linvel.x < -f32::EPSILON {
                // Moving left
                let idx = sprite_index(&MOVE_LEFT_INDICES, atlas.index);
                atlas.index = MOVE_LEFT_INDICES[idx];
            } else if climber.climbing {
                // Climbing
                let idx = if velocity.linvel.y > f32::EPSILON || velocity.linvel.y < -f32::EPSILON {
                    // Moving
                    sprite_index(&CLIMB_INDICES, atlas.index)
                } else {
                    // Doesn't move during climbing
                    0
                };
                atlas.index = CLIMB_INDICES[idx];
            } else {
                // Doesn't move
                atlas.index = 16;
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
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
) {
    const MOVE_SPEED: f32 = 160.;
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
                commands.entity(player_entity).insert((
                    Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY),
                    Blink::new(Duration::from_secs_f32(0.15)),
                ));
            }
        }
    }
}
