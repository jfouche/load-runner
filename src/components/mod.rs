mod level;
pub use level::*;

mod character;
pub use character::*;

mod player;
pub use player::*;

mod enemy;
pub use enemy::*;

mod item;
pub use item::*;

mod states;
pub use states::*;

use bevy::prelude::*;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
