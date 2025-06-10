mod character_plugin;
mod death_menu;
mod end_level_menu;
mod enemy_plugin;
mod hud_plugin;
mod item_plugin;
mod level_plugin;
mod pause_menu;
mod player_plugin;
mod popup;

pub use plugin::InGamePlugins;

mod plugin {

    use super::*;
    use crate::{
        cursor::{grab_cursor, ungrab_cursor},
        in_game::popup::PopupWithImages,
        schedule::{GameState, InGameSet, InGameState},
        utils::{blink::Blink, invulnerable::Invulnerable},
    };
    use bevy::{app::PluginGroupBuilder, prelude::*};
    use bevy_rapier2d::prelude::*;

    pub struct InGamePlugins;

    impl PluginGroup for InGamePlugins {
        fn build(self) -> PluginGroupBuilder {
            PluginGroupBuilder::start::<Self>()
                .add(death_menu::plugin)
                .add(character_plugin::character_plugin)
                .add(enemy_plugin::enemy_plugin)
                .add(hud_plugin::hud_plugin)
                .add(level_plugin::level_plugin)
                .add(player_plugin::player_plugin)
                .add(pause_menu::pause_menu_plugin)
                .add(item_plugin::item_plugin)
                .add(end_level_menu::end_level_menu_plugin)
                .add(popup::popup_plugin)
                .add(in_game_plugin)
        }
    }

    fn in_game_plugin(app: &mut App) {
        app.add_systems(Startup, stop_physics)
            .add_systems(OnEnter(GameState::InGame), grab_cursor)
            .add_systems(OnExit(GameState::InGame), (ungrab_cursor, reset_physics))
            .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
            .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
            .add_systems(OnEnter(InGameState::Pause), pause)
            .add_systems(OnExit(InGameState::Pause), unpause)
            .add_systems(OnEnter(InGameState::ShowPopup), pause)
            .add_systems(OnExit(InGameState::ShowPopup), unpause)
            .add_systems(Update, switch_to_pause.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (
                    enter_popup_state.in_set(InGameSet::EntityUpdate),
                    exit_popup_state.run_if(in_state(InGameState::ShowPopup)),
                ),
            );
    }

    fn switch_to_pause(mut state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
        if keys.just_pressed(KeyCode::Escape) {
            state.set(InGameState::Pause);
        }
    }

    fn pause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
        for mut blink in &mut blinks {
            blink.pause(true);
        }
        for mut invulnerable in &mut invulnerables {
            invulnerable.pause(true);
        }
    }

    fn unpause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
        for mut blink in &mut blinks {
            blink.pause(false);
        }
        for mut invulnerable in &mut invulnerables {
            invulnerable.pause(false);
        }
    }

    fn start_physics(mut confs: Query<&mut RapierConfiguration, With<DefaultRapierContext>>) {
        if let Ok(mut conf) = confs.single_mut() {
            conf.physics_pipeline_active = true;
            conf.query_pipeline_active = true;
        }
    }

    fn stop_physics(mut confs: Query<&mut RapierConfiguration, With<DefaultRapierContext>>) {
        if let Ok(mut conf) = confs.single_mut() {
            conf.physics_pipeline_active = false;
            conf.query_pipeline_active = false;
        }
    }

    fn reset_physics(mut commands: Commands) {
        commands.insert_resource(Events::<CollisionEvent>::default());
        commands.insert_resource(Events::<ContactForceEvent>::default());
    }

    fn enter_popup_state(
        query: Query<(), Added<PopupWithImages>>,
        mut in_game_state: ResMut<NextState<InGameState>>,
    ) {
        if query.single().is_ok() {
            in_game_state.set(InGameState::ShowPopup);
        }
    }

    fn exit_popup_state(
        query: RemovedComponents<PopupWithImages>,
        mut in_game_state: ResMut<NextState<InGameState>>,
    ) {
        if !query.is_empty() {
            in_game_state.set(InGameState::Running);
        }
    }
}
