use crate::{
    components::{
        character::Life,
        despawn_all,
        item::{ItemAssets, Items},
        player::Player,
    },
    schedule::{GameState, InGameSet},
    theme::widget,
    ui::ProgressBar,
};
use bevy::prelude::*;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (spawn_player_items, spawn_player_life),
    )
    .add_systems(OnExit(GameState::InGame), despawn_all::<Hud>)
    .add_systems(
        Update,
        (update_player_items, update_player_life).in_set(InGameSet::EntityUpdate),
    );
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudPlayerItems;

#[derive(Component)]
struct HudPlayerLife;

fn spawn_player_items(mut commands: Commands) {
    commands.spawn((
        Name::new("HudPlayerItems"),
        HudPlayerItems,
        Hud,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..widget::hsizer()
        },
        BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.2)),
    ));
}

fn spawn_player_life(mut commands: Commands) {
    commands.spawn((
        Name::new("HudPlayerLife"),
        HudPlayerLife,
        Hud,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            width: Val::Px(250.0),
            height: Val::Px(16.0),
            ..Default::default()
        },
        ProgressBar::new(0.0, 10.0, 10.0).with_colors(Color::BLACK, Srgba::RED.into()),
    ));
}

fn update_player_items(
    mut commands: Commands,
    players: Query<&Items, (With<Player>, Changed<Items>)>,
    huds: Query<Entity, With<HudPlayerItems>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(items) = players.single() {
        let hud = huds.single().expect("HudPlayerItems");

        commands
            .entity(hud)
            // clear all spawned items
            .despawn_related::<Children>()
            // add all items
            .with_children(|parent| {
                for &item in items.iter() {
                    parent.spawn((Name::new(format!("{item:?}")), assets.image_node(item)));
                }
            });
    };
}

fn update_player_life(
    life: Query<&Life, With<Player>>,
    mut progressbars: Query<&mut ProgressBar, With<HudPlayerLife>>,
) {
    if let Ok(life) = life.single() {
        for mut progressbar in progressbars.iter_mut() {
            progressbar.set_value(life.get() as f32);
        }
    }
}
