use super::collisions::*;
use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn item_plugin(app: &mut App) {
    app.add_systems(Update, open_chest.in_set(InGameSet::CollisionDetection));
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
            commands.entity(chest_entity).despawn_recursive();
        });
}

fn display_collision_events(mut collisions: EventReader<CollisionEvent>) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {}
            CollisionEvent::Stopped(e1, e2, flag) => {}
        }
    }
}
