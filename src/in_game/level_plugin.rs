use super::{collisions::*, popup::*};
use crate::{components::*, schedule::InGameSet, ui::*};
use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn level_plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(DIRT_INT_CELL)
        .register_ldtk_int_cell::<LadderBundle>(LADDER_INT_CELL)
        .register_ldtk_int_cell::<WallBundle>(STONE_INT_CELL)
        .register_ldtk_int_cell::<WaterBundle>(WATER_INT_CELL)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<MobBundle>("Mob")
        .register_ldtk_entity::<ChestBundle>("Chest")
        .register_ldtk_entity::<DoorBundle>("Door")
        .register_ldtk_entity::<EndLevelBundle>("End")
        // LoadLevel
        .add_systems(OnEnter(InGameState::LoadLevel), (show_level, spawn_level))
        .add_systems(
            Update,
            spawn_wall_collision.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            start_level_after_fading.run_if(in_state(InGameState::LoadLevel)),
        )
        // InGame
        .add_systems(
            Update,
            update_level_selection.in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (open_door, end_level).in_set(InGameSet::CollisionDetection),
        )
        .add_systems(Update, restart_level.in_set(InGameSet::UserInput));
}

const END_LEVEL_FADE_COLOR: Color = Color::srgba(0.0, 0.0, 0.8, 1.0);

fn show_level(mut commands: Commands) {
    info!("show_level()");
    commands.spawn(FaderBundle::new(END_LEVEL_FADE_COLOR, Color::NONE, 2.0));
}

/// wait for fader to finish, and start running
fn start_level_after_fading(
    mut commands: Commands,
    mut events: EventReader<FaderFinishEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for event in events.read() {
        if let Some(mut fader) = commands.get_entity(event.entity) {
            info!("start_level() - despawn({:?})", event.entity);
            fader.despawn();
        }
        in_game_state.set(InGameState::Running);
    }
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
) {
    match ldtk_projects.get_single() {
        Ok(world_entity) => {
            // A project is already loaded, respawn it
            commands.entity(world_entity).insert(Respawn);
        }
        Err(QuerySingleError::NoEntities(_)) => {
            // Spawn a new project
            let ldtk_handle = asset_server.load("load-runner.ldtk");
            commands.spawn(LdtkWorldBundle {
                ldtk_handle,
                ..Default::default()
            });
        }
        Err(e) => panic!("{e:?}"),
    }
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let mut level_colliders = LevelColliders::new();
    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_colliders.add_coord(grandparent.get(), grid_coords);
        }
    });

    if !wall_query.is_empty() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        for (level_entity, level_iid) in &level_query {
            let level = ldtk_project
                .as_standalone()
                .get_loaded_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            let LayerInstance {
                c_wid: width,
                c_hei: height,
                grid_size,
                ..
            } = level.layer_instances()[COLLISIONS_LAYER];

            let colliders = level_colliders.combine(&level_entity, width, height, grid_size);

            commands.entity(level_entity).with_children(|level| {
                // Spawn colliders for every rectangle..
                // Making the collider a child of the level serves two purposes:
                // 1. Adjusts the transforms to be relative to the level for free
                // 2. the colliders will be despawned automatically when levels unload
                for collider in colliders {
                    level.spawn(collider);
                }
            });
        }
    }
}

fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Project should be loaded if level has spawned");

    for (level_iid, level_transform) in &level_query {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");
        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + level.px_wid as f32,
                level_transform.translation.y + level.px_hei as f32,
            ),
        };

        for player_transform in &player_query {
            if player_transform.translation.x < level_bounds.max.x
                && player_transform.translation.x > level_bounds.min.x
                && player_transform.translation.y < level_bounds.max.y
                && player_transform.translation.y > level_bounds.min.y
                && !level_selection.is_match(&LevelIndices::default(), level)
            {
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for level_entity in &level_query {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}

fn open_door(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Items), With<Player>>,
    doors: Query<&Items, (With<Door>, Without<Player>)>,
    assets: Res<ItemAssets>,
) {
    let (player_entity, mut player_items) = players.get_single_mut().expect("Player");
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| doors.get_either(e1, e2))
        .filter(|(_expected_items, _door_entity, other_entity)| player_entity == *other_entity)
        .for_each(|(expected_items, door_entity, _player_entity)| {
            if player_items.contains_items(expected_items) {
                info!("Player open door");
                player_items.remove_items(expected_items);
                commands.entity(door_entity).despawn_recursive();
            } else {
                // Show a popup that shows the expected items to open the door
                let mut popup_bundle =
                    PopupBundle::new("Closed door", "You should have the following items");
                for &item in expected_items.iter() {
                    popup_bundle.add_image(assets.image_components(item));
                }
                commands.spawn(popup_bundle);
            }
        });
}

fn end_level(
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<Entity, With<Player>>,
    end_levels: Query<&EndLevel>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    let player_entity = players.get_single_mut().expect("Player");
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| end_levels.get_either(e1, e2))
        .filter(|(_, _end_entity, other_entity)| player_entity == *other_entity)
        .for_each(|(_, _end_entity, _player_entity)| {
            info!("Player end level");
            in_game_state.set(InGameState::PlayerEndedLevel);
        });
}
