use crate::components::*;
use crate::schedule::InGameSet;
use crate::ui::*;
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
        NodeBundle {
            background_color: Color::rgba(0.3, 0.3, 0.3, 0.2).into(),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..hsizer().style
            },
            ..Default::default()
        },
    ));
}

fn spawn_player_life(mut commands: Commands) {
    commands.spawn((
        Name::new("HudPlayerLife"),
        HudPlayerLife,
        Hud,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(250.0),
                height: Val::Px(16.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ProgressBar::new(0.0, 10.0, 10.0).with_colors(Color::BLACK, Color::RED),
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

fn update_player_life(
    life: Query<&Life, With<Player>>,
    mut progressbars: Query<&mut ProgressBar, With<HudPlayerLife>>,
) {
    if let Ok(life) = life.get_single() {
        for mut progressbar in progressbars.iter_mut() {
            progressbar.set_value(life.get() as f32);
        }
    }
}
