#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{components::*, cursor::*, schedule::InGameSet};
use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use bevy_ecs_ldtk::{
    prelude::*,
    utils::{translation_to_grid_coords, translation_to_ldtk_pixel_coords},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::new(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_systems(
        Update,
        (toggle_grab, display_player_items).in_set(InGameSet::UserInput),
    )
    .add_systems(
        Update,
        (
            // display_collision_events,
            display_player_position.run_if(on_timer(Duration::from_secs(1)))
        )
        .in_set(InGameSet::EntityUpdate),
    )
    // States
    .add_systems(
        Update,
        (
            state_transition::<GameState>,
            state_transition::<InGameState>,
        ),
    );
}

fn toggle_grab(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::KeyG) {
            match window.cursor.grab_mode {
                bevy::window::CursorGrabMode::None => {
                    set_grab_cursor(window, true);
                }
                _ => {
                    set_grab_cursor(window, false);
                }
            }
        }
    }
}

fn display_collision_events(mut collisions: EventReader<CollisionEvent>, names: Query<DebugName>) {
    let get_name = |e| {
        names
            .get(e)
            .map(|dn| format!("{dn:?}"))
            .unwrap_or(format!("{e:?}"))
    };

    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {
                let n1 = get_name(*e1);
                let n2 = get_name(*e2);
                info!("CollisionEvent::Started({n1}, {n2}, {flag:?})");
            }
            CollisionEvent::Stopped(e1, e2, flag) => {
                let n1 = get_name(*e1);
                let n2 = get_name(*e2);
                info!("CollisionEvent::Stopped({n1}, {n2}, {flag:?})");
            }
        }
    }
}

fn display_player_position(
    players: Query<&Transform, With<Player>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        level_query
            .iter()
            .filter_map(|(transform, iid)| {
                let ldtk_project = ldtk_project_assets.get(ldtk_projects.single())?;
                let level = ldtk_project.get_raw_level_by_iid(&iid.to_string())?;
                let layer_info = level.layer_instances.as_ref()?.get(COLLISIONS_LAYER)?;
                level_selection
                    .is_match(&LevelIndices::default(), level)
                    .then_some((transform, layer_info))
            })
            .for_each(|(level_transform, layer_info)| {
                let translation =
                    player_transform.translation.xy() - level_transform.translation.xy();
                let character_coord =
                    translation_to_grid_coords(translation, IVec2::splat(layer_info.grid_size));
                info!("update_in_water: coord: {character_coord:?}");
            });
    }
}

fn display_states(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) {
    info!(
        "GameState::{:?} - InGameState::{:?}",
        **game_state, **in_game_state
    );
}

fn state_transition<S: States>(mut events: EventReader<StateTransitionEvent<S>>) {
    for event in events.read() {
        let name = std::any::type_name::<S>();
        info!("{name} : {event:?}");
    }
}

fn display_player_items(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Items, &EntityInstance), With<Player>>,
) {
    for (items, entity_instance) in &mut query {
        if input.just_pressed(KeyCode::KeyI) {
            dbg!(&items);
            dbg!(&entity_instance);
        }
    }
}
