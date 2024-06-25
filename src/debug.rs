use crate::{components::*, cursor::*, schedule::InGameSet};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::EntityInstance;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

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
        (display_collision_events,).after(InGameSet::EntityUpdate),
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

#[allow(dead_code)]
fn display_states(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) {
    info!(
        "GameState::{:?} - InGameState::{:?}",
        **game_state, **in_game_state
    );
}

fn state_transition<S: States>(mut events: EventReader<StateTransitionEvent<S>>) {
    for event in events.read() {
        let name = std::any::type_name::<S>();
        info!("{name} : {:?} => {:?}", event.before, event.after);
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
