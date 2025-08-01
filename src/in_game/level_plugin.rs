use crate::{
    components::{
        enemy::LdtkMobBundle,
        item::{ItemAssets, Items, LdtkChestBundle},
        level::{
            level_collider, ColliderCell, Destroyed, Destructible, Door, EndLevel, LdtkDirtCell,
            LdtkDoorBundle, LdtkEndLevelBundle, LdtkLadderCell, LdtkStoneCell, LdtkWaterCell,
            LevelCollider, LevelColliders, LevelData, LoadedLevelExt, UpdateCollidersEvent,
            DIRT_INT_CELL, LADDER_INT_CELL, STONE_INT_CELL, WATER_INT_CELL,
        },
        player::{DigEvent, LdtkPlayerBundle, Player},
    },
    in_game::popup_with_images::popup_with_images,
    schedule::{GameState, InGameSet, InGameState},
    theme::widget,
    ui::fade::{fader, FaderFinishEvent},
    utils::collisions::{start_event_filter, QueryEither},
};
use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::tiles::TileVisible;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

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
        .register_ldtk_int_cell::<LdtkDirtCell>(DIRT_INT_CELL)
        .register_ldtk_int_cell::<LdtkLadderCell>(LADDER_INT_CELL)
        .register_ldtk_int_cell::<LdtkStoneCell>(STONE_INT_CELL)
        .register_ldtk_int_cell::<LdtkWaterCell>(WATER_INT_CELL)
        .register_ldtk_entity::<LdtkPlayerBundle>("Player")
        .register_ldtk_entity::<LdtkMobBundle>("Mob")
        .register_ldtk_entity::<LdtkChestBundle>("Chest")
        .register_ldtk_entity::<LdtkDoorBundle>("Door")
        .register_ldtk_entity::<LdtkEndLevelBundle>("End")
        // LevelLoading
        .add_systems(
            OnEnter(InGameState::LevelLoading),
            (spawn_loading_screen, spawn_level),
        )
        .add_systems(
            Update,
            wait_for_end_of_level_loading.run_if(in_state(InGameState::LevelLoading)),
        )
        // LevelLoading
        .add_systems(OnEnter(InGameState::LevelLoaded), show_level)
        .add_systems(
            Update,
            initialize_level_collisions.run_if(in_state(GameState::InGame)),
        )
        // InGame
        .add_systems(
            Update,
            update_level_based_on_player_pos.in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (open_door, end_level).in_set(InGameSet::CollisionDetection),
        )
        .add_systems(Update, restart_level.in_set(InGameSet::UserInput))
        .add_observer(run_level_after_fading)
        .add_observer(on_dig)
        .add_observer(recalculate_level_collisions);
}

#[derive(Component)]
struct LoadingScreen;

fn loading_screen() -> impl Bundle {
    (
        LoadingScreen,
        widget::ui_root("LoadingScreen"),
        BackgroundColor(LOADING_SCREEN_BACKGROUND_COLOR),
    )
}

const LOADING_SCREEN_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.8, 1.0);

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((loading_screen(), StateScoped(InGameState::LevelLoading)));
}

fn show_level(mut commands: Commands) {
    commands.spawn(fader(LOADING_SCREEN_BACKGROUND_COLOR, Color::NONE, 2.0));
}

/// wait for fader to finish, and start running game
fn run_level_after_fading(
    trigger: Trigger<FaderFinishEvent>,
    mut commands: Commands,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if let Ok(mut fader) = commands.get_entity(trigger.target()) {
        fader.despawn();
    }
    in_game_state.set(InGameState::Running);
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
) {
    match ldtk_projects.single() {
        Ok(world_entity) => {
            // A project is already loaded, respawn it
            commands.entity(world_entity).insert(Respawn);
        }
        Err(QuerySingleError::NoEntities(_)) => {
            // Spawn a new project
            commands.spawn((
                LdtkWorldBundle {
                    ldtk_handle: asset_server.load("load-runner.ldtk").into(),
                    ..Default::default()
                },
                Name::new("MapWorld"),
            ));
        }
        Err(e) => panic!("{e:?}"),
    }
}

/// Wait for all [LevelEvent::Spawned] required by all [LevelEvent::SpawnTriggered]
/// and set in the [InGameState::LevelLoaded].
fn wait_for_end_of_level_loading(
    mut events: EventReader<LevelEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut progress: Local<HashSet<LevelIid>>,
) {
    for event in events.read() {
        match event {
            LevelEvent::SpawnTriggered(liid) => {
                info!("LevelEvent::SpawnTriggered({liid})");
                progress.insert(liid.clone());
            }
            LevelEvent::Spawned(liid) => {
                info!("LevelEvent::Spawned({liid})");
                progress.remove(liid);
                if progress.is_empty() {
                    in_game_state.set(InGameState::LevelLoaded);
                }
            }
            _ => {}
        }
    }
}

/// Spawns collisions for the walls of a level
///
/// You could just insert a Collider in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
fn initialize_level_collisions(
    mut commands: Commands,
    collider_cells: Query<(&GridCoords, &ChildOf), Added<ColliderCell>>,
    parents: Query<&ChildOf, Without<ColliderCell>>,
    levels: Query<LevelData>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    if collider_cells.is_empty() {
        return Ok(());
    }

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .ok_or("Project should be loaded if level has spawned")?
        .as_standalone();

    let level_colliders = LevelColliders::new(collider_cells, &parents);
    for level_data in &levels {
        let level = level_data.level(ldtk_project)?;
        let layer = level.collision_layer()?;
        for rect in level_colliders.rectangles(&level_data.entity, layer.c_wid, layer.c_hei) {
            commands.spawn((
                level_collider(rect, layer.grid_size),
                ChildOf(level_data.entity),
            ));
        }
    }
    Ok(())
}

fn update_level_based_on_player_pos(
    levels: Query<(&LevelIid, &Transform), Without<Player>>,
    players: Query<&Transform, With<Player>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .ok_or("Project should be loaded if level has spawned")?;

    let player_pos = players.single()?.translation.xy();

    levels
        .iter()
        // Get level bounds
        .filter_map(|(liid, transform)| {
            let level = ldtk_project.get_raw_level_by_iid(&liid.to_string())?;
            let min = transform.translation.xy();
            let max = min + vec2(level.px_wid as f32, level.px_hei as f32);
            Some((level, Rect { min, max }))
        })
        // Check if player is in level
        .filter_map(|(level, level_bounds)| level_bounds.contains(player_pos).then_some(level))
        // Select level
        .for_each(|level| *level_selection = LevelSelection::iid(level.iid.clone()));
    Ok(())
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
    let (player_entity, mut player_items) = players.single_mut().expect("Player");
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| doors.get_either(e1, e2))
        .filter(|(_expected_items, _door_entity, other_entity)| player_entity == *other_entity)
        .for_each(|(expected_items, door_entity, _player_entity)| {
            if player_items.contains_items(expected_items) {
                info!("Player open door");
                player_items.remove_items(expected_items);
                commands.entity(door_entity).despawn();
            } else {
                // Show a popup that shows the expected items to open the door
                let images = expected_items
                    .iter()
                    .map(|&i| assets.image_node(i))
                    .collect::<Vec<_>>();
                commands.spawn(popup_with_images(
                    "Closed door",
                    "You should have the following items",
                    images,
                ));
            }
        });
}

fn end_level(
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<Entity, With<Player>>,
    end_levels: Query<&EndLevel>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    let player_entity = players.single_mut().expect("Player");
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

fn on_dig(
    trigger: Trigger<DigEvent>,
    mut commands: Commands,
    mut cells: Query<&mut TileVisible, With<Destructible>>,
) {
    warn!("DigEvent {}", trigger.target());
    if let Ok(mut visible) = cells.get_mut(trigger.target()) {
        visible.0 = false;
        commands.entity(trigger.target()).insert(Destroyed);
        commands.trigger(UpdateCollidersEvent);
    }
}

fn recalculate_level_collisions(
    _trigger: Trigger<UpdateCollidersEvent>,
    mut commands: Commands,
    colliders: Query<Entity, With<LevelCollider>>,
    collider_cells: Query<(&GridCoords, &ChildOf), (With<ColliderCell>, Without<Destroyed>)>,
    parents: Query<&ChildOf, Without<ColliderCell>>,
    levels: Query<LevelData>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    warn!("recalculate_level_collisions");

    let existing_colliders = colliders.iter().collect::<Vec<_>>();

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .ok_or("Project should be loaded if level has spawned")?
        .as_standalone();

    let level_colliders = LevelColliders::new(collider_cells, &parents);
    for level_data in &levels {
        let level = level_data.level(ldtk_project)?;
        let layer = level.collision_layer()?;
        for rect in level_colliders.rectangles(&level_data.entity, layer.c_wid, layer.c_hei) {
            commands.spawn((
                level_collider(rect, layer.grid_size),
                ChildOf(level_data.entity),
            ));
        }
    }

    // remove old colliders
    for e in existing_colliders {
        commands.entity(e).despawn();
    }

    Ok(())
}
