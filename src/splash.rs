use crate::{
    asset_tracking::ResourceHandles, components::despawn_all, cursor::ungrab_cursor,
    schedule::GameState,
};
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Splash),
        (spawn_splash_screen, ungrab_cursor),
    )
    .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
    .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

#[derive(Component)]
struct SplashScreen;

fn spash_screen() -> impl Bundle {
    (
        SplashScreen,
        Name::new("SplashScreen"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            (
                Text("Load-Runner".into()),
                TextFont::from_font_size(80.),
                TextColor(Color::WHITE)
            ),
            (
                Text("Press any key to continue".into()),
                TextFont::from_font_size(16.),
                TextColor(Color::BLACK)
            ),
        ],
    )
}

#[derive(Component)]
struct SplashScreenMessage;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

fn spawn_splash_screen(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.spawn(spash_screen());
}

// fn display_continue(mut messages: Query<&mut Text, With<SplashScreenMessage>>) {
//     for mut text in &mut messages {
//         text.sections[0].value = "Press any key to continue".into();
//     }
// }

#[allow(clippy::collapsible_if)]
fn goto_main_menu(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    resources: Res<ResourceHandles>,
) {
    if resources.is_all_done() {
        if keys.get_pressed().len() != 0 || mouse.pressed(MouseButton::Left) {
            game_state.set(GameState::Menu);
        }
    }
}
