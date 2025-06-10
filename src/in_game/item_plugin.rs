use crate::{
    components::{
        item::{Chest, ItemAssets, Items},
        player::Player,
    },
    in_game::popup_with_images::*,
    schedule::InGameSet,
    utils::collisions::{start_event_filter, QueryEither},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn item_plugin(app: &mut App) {
    app.register_type::<Items>()
        .init_resource::<ItemAssets>()
        .add_systems(Update, open_chest.in_set(InGameSet::CollisionDetection));
}

fn open_chest(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(Entity, &mut Items), With<Player>>,
    chests: Query<&Items, (With<Chest>, Without<Player>)>,
    assets: Res<ItemAssets>,
) {
    let (player_entity, mut player_items) = players.single_mut().expect("Player");
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
            commands.entity(chest_entity).despawn();

            // Show a popup with chest items
            let images = chest_items
                .iter()
                .map(|&i| assets.image_node(i))
                .collect::<Vec<_>>();
            commands.spawn(popup_with_images("Chest opened", "You found", images));
        });
}
