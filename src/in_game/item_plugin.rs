use super::collisions::*;
use crate::{components::*, in_game::popup::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn item_plugin(app: &mut App) {
    app.register_type::<Items>()
        .add_systems(Startup, load_assets)
        .add_systems(Update, open_chest.in_set(InGameSet::CollisionDetection));
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("atlas/MV Icons Complete Sheet Free - ALL.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 16, 95, None, None);
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
    assets: Res<ItemAssets>,
) {
    let (player_entity, mut player_items) = players.get_single_mut().expect("Player");
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| chests.get_either(e1, e2))
        .filter(|(_items, _chest_entity, other_entity)| player_entity == *other_entity)
        .for_each(|(chest_items, chest_entity, _player_entity)| {
            info!("Player open chest");
            // Player get chest items
            for i in chest_items.iter() {
                player_items.add(*i);
            }

            // Remove the chest
            commands.entity(chest_entity).despawn_recursive();

            // Show a popup with chest items
            let mut popup_bundle = PopupBundle::new("Chest opened", "You found");
            for &item in chest_items.iter() {
                popup_bundle.add_image(assets.image_components(item));
            }
            commands.spawn(popup_bundle);
        });
}
