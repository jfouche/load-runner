use super::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub enum Item {
    Key,
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(pub Vec<String>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        warn!("Items: {}", entity_instance.identifier);
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect(),
        )
    }
}

#[derive(Copy, Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Chest;

#[derive(Clone, Bundle, LdtkEntity)]
pub struct ChestBundle {
    tag: Chest,
    name: Name,
    // #[from_entity_instance]
    // items: Items,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collider_bundle: ColliderBundle,
}

impl Default for ChestBundle {
    fn default() -> Self {
        ChestBundle {
            tag: Chest,
            name: Name::new("Chest"),
            // items: Items::default(),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
        }
    }
}
