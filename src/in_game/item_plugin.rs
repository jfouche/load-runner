use super::collisions::*;
use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn item_plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(Update, open_chest.in_set(InGameSet::CollisionDetection));
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("atlas/MV Icons Complete Sheet Free - ALL.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 16, 95, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let assets = ItemAssets {
        texture,
        texture_atlas_layout,
    };
    commands.insert_resource(assets);
}

fn open_chest(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Items), With<Player>>,
    chests: Query<&Items, (With<Chest>, Without<Player>)>,
) {
    let (player_entity, mut player_items) = players.get_single_mut().expect("Player");
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| chests.get_either(e1, e2))
        .filter(|(_items, _chest_entity, other_entity)| player_entity == *other_entity)
        .for_each(|(chest_items, chest_entity, _player_entity)| {
            info!("Player open chest");
            for i in &chest_items.0 {
                player_items.0.push(*i);
            }
            commands.entity(chest_entity).despawn_recursive();
        });
}
