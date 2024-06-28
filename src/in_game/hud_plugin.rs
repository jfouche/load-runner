use crate::components::*;
use crate::schedule::InGameSet;
use crate::ui::*;
use bevy::prelude::*;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_player_items)
        .add_systems(OnExit(GameState::InGame), despawn_all::<HudPlayerItems>)
        .add_systems(Update, update_player_items.in_set(InGameSet::EntityUpdate));
}

#[derive(Component)]
struct HudPlayerItems;

fn spawn_player_items(mut commands: Commands) {
    commands.spawn((
        Name::new("HudPlayerItems"),
        HudPlayerItems,
        NodeBundle {
            background_color: Color::rgba(0.3, 0.3, 0.3, 0.2).into(),
            style: Style {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..hsizer().style
            },
            ..Default::default()
        },
    ));
}

fn update_player_items(
    mut commands: Commands,
    players: Query<&Items, (With<Player>, Changed<Items>)>,
    huds: Query<Entity, With<HudPlayerItems>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(items) = players.get_single() {
        let hud = huds.get_single().expect("HudPlayerItems");

        // clear all spawned items
        let mut cmd = commands.entity(hud);
        cmd.despawn_descendants();

        // add all items
        cmd.with_children(|parent| {
            for &item in &items.0 {
                parent.spawn((Name::new(format!("{item:?}")), assets.image_bundle(item)));
            }
        });
    };
}
