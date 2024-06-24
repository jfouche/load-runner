use crate::components::*;
use crate::schedule::InGameSet;
use crate::ui::*;
use bevy::prelude::*;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_player_items)
        .add_systems(OnExit(GameState::InGame), despawn_all::<Player>)
        .add_systems(Update, update_player_items.in_set(InGameSet::EntityUpdate));
}

#[derive(Component)]
struct HudPlayerItems;

fn spawn_player_items(mut commands: Commands) {
    commands
        .spawn((
            Name::new("HudPlayerItems"),
            NodeBundle {
                style: fullscreen_style(),
                ..Default::default()
            },
        ))
        .with_children(|wnd| {
            wnd.spawn(NodeBundle {
                background_color: Color::rgba(0.3, 0.3, 0.3, 0.5).into(),
                style: Style {
                    top: Val::Px(10.0),
                    height: Val::Px(64.0),
                    width: Val::Px(500.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with_children(|panel| {
            panel.spawn((
                HudPlayerItems,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
            ));
        });
}

fn update_player_items(
    players: Query<&Items, With<Player>>,
    mut texts: Query<&mut Text, With<HudPlayerItems>>,
) {
    let items = players.get_single().expect("Player");
    let mut text = texts.get_single_mut().expect("HudPlayerItems");
    text.sections[0].value = items.0.join(" - ");
}
