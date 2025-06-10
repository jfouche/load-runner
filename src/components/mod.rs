pub mod character;
pub mod enemy;
pub mod item;
pub mod level;
pub mod player;

pub use utils::{despawn_all, GROUP_ENEMY, GROUP_PLAYER};

mod utils {
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::Group;

    pub const GROUP_PLAYER: Group = Group::GROUP_1;
    pub const GROUP_ENEMY: Group = Group::GROUP_2;

    /// Generic system that takes a component as a parameter, and will despawn all entities with that component
    pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
        for entity in &to_despawn {
            commands.entity(entity).despawn();
        }
    }
}
