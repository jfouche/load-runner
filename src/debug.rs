#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{
    components::{item::Items, level::COLLISIONS_LAYER, player::Player},
    cursor::*,
    schedule::*,
    utils::iter_ext::IterExt,
};
use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin, input::common_conditions::input_just_pressed,
    prelude::*, time::common_conditions::on_timer, window::PrimaryWindow,
};
use bevy_ecs_ldtk::{
    prelude::*,
    utils::{translation_to_grid_coords, translation_to_ldtk_pixel_coords},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiContextPass, EguiPlugin},
    bevy_inspector::{self, Filter},
    egui,
    quick::WorldInspectorPlugin,
    DefaultInspectorConfigPlugin,
};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        FpsOverlayPlugin::default(),
        DefaultInspectorConfigPlugin,
        WorldInspectorPlugin::new().run_if(debug_is_active),
        RapierDebugRenderPlugin::default(),
    ))
    .init_resource::<UiDebugOptions>()
    .insert_resource(DebugMode(true))
    .add_systems(EguiContextPass, inspector_ui.run_if(debug_is_active))
    .add_systems(
        Update,
        (
            toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
            toggle_debug_mode.run_if(input_just_pressed(KeyCode::Backspace)),
        ),
    )
    .add_systems(
        Update,
        (
            toggle_grab.run_if(input_just_pressed(KeyCode::KeyG)),
            display_player_items.run_if(input_just_pressed(KeyCode::KeyI)),
        )
            .in_set(InGameSet::UserInput),
    )
    .add_systems(
        Update,
        (
            display_collision_events,
            // display_player_position.run_if(on_timer(Duration::from_secs(1))),
        )
            .run_if(debug_is_active)
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

#[derive(Resource, Deref, DerefMut)]
struct DebugMode(bool);

fn debug_is_active(debug: Res<DebugMode>) -> bool {
    **debug
}

fn inspector_ui(world: &mut World) {
    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .cloned()
    else {
        return;
    };
    egui::Window::new("World").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let filter = Filter::<(Without<ChildOf>, Without<Observer>)>::from_ui_fuzzy(
                ui,
                egui::Id::new("Load-Runner Debug UI"),
            );
            bevy_inspector::ui_for_entities_filtered(world, ui, true, &filter);
            ui.allocate_space(ui.available_size());
        });
    });
}

fn toggle_debug_mode(mut mode: ResMut<DebugMode>) {
    **mode = !**mode;
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_grab(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(window) = primary_window.single_mut() {
        let grab = match window.cursor_options.grab_mode {
            bevy::window::CursorGrabMode::None => true,
            _ => false,
        };
        set_grab_cursor(window, grab);
    }
}

fn display_collision_events(
    mut collisions: EventReader<CollisionEvent>,
    names: Query<NameOrEntity>,
) {
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
    ldtk_projects: Query<&LdtkProjectHandle>,
    levels: Query<(&Transform, &LevelIid), Without<Player>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) -> Result {
    let player_transform = players.single()?;
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .ok_or(BevyError::from("Project should exist"))?;

    let player_coord = levels
        .iter()
        .filter_map(|(transform, iid)| {
            let level = ldtk_project.get_raw_level_by_iid(&iid.to_string())?;
            let layer_info = level.layer_instances.as_ref()?.get(COLLISIONS_LAYER)?;
            level_selection
                .is_match(&LevelIndices::default(), level)
                .then_some((transform, layer_info))
        })
        .map(|(level_transform, layer_info)| {
            let translation = player_transform.translation.xy() - level_transform.translation.xy();
            translation_to_grid_coords(translation, IVec2::splat(layer_info.grid_size))
        })
        .single()?;
    info!("player coords: {player_coord:?}");

    Ok(())
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

fn display_player_items(mut query: Query<(&Items, &EntityInstance), With<Player>>) {
    for (items, entity_instance) in &mut query {
        dbg!(&items);
        dbg!(&entity_instance);
    }
}
