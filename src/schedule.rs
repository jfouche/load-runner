use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameLoadingSet {
    CreateLevel,
    SpawnLevelEntities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollisionDetection,
    DespawnEntities,
}

/// Represent the Game state
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
}

/// Represent the state while in game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Running,
    Pause,
    PlayerEndedLevel,
    PlayerDied,
    ShowPopup,
    LoadLevel,
}

pub fn schedule_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .init_state::<InGameState>()
        .configure_sets(
            OnEnter(InGameState::LoadLevel),
            (
                InGameLoadingSet::CreateLevel,
                // apply_deffer will be added here
                InGameLoadingSet::SpawnLevelEntities,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                // apply_deffer will be added here
                InGameSet::UserInput,
                InGameSet::EntityUpdate,
                InGameSet::CollisionDetection,
            )
                .chain()
                .run_if(game_is_running),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            ApplyDeferred
                .after(InGameLoadingSet::CreateLevel)
                .before(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(
            Update,
            ApplyDeferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        )
        .add_systems(OnEnter(GameState::InGame), new_game)
        .add_systems(OnExit(GameState::InGame), end_game);
}

fn game_is_running(
    game_state: Res<State<GameState>>,
    in_game_state: Res<State<InGameState>>,
) -> bool {
    *game_state == GameState::InGame && *in_game_state == InGameState::Running
}

fn new_game(
    // mut current_level: ResMut<CurrentLevel>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    // **current_level = 0;
    in_game_state.set(InGameState::LoadLevel);
}

fn end_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Disabled);
}
