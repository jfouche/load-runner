use super::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Item {
    Unknown,
    Knife,
    Boots,
    Key,
}

impl From<&String> for Item {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "Knife" => Item::Knife,
            "Boots" => Item::Boots,
            "Key" => Item::Key,
            _ => Item::Unknown,
        }
    }
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(pub Vec<Item>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .map(|s| s.into())
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
    #[from_entity_instance]
    items: Items,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collider_bundle: ColliderBundle,
}

impl Default for ChestBundle {
    fn default() -> Self {
        ChestBundle {
            tag: Chest,
            name: Name::new("Chest"),
            items: Items::default(),
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

#[derive(Resource)]
pub struct ItemAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl ItemAssets {
    pub fn image_bundle(&self, item: Item) -> AtlasImageBundle {
        let index = match item {
            Item::Boots => 18,
            Item::Key => 83,
            Item::Knife => 19,
            Item::Unknown => 0,
        };
        AtlasImageBundle {
            texture_atlas: TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
            image: UiImage::new(self.texture.clone()),
            ..default()
        }
    }
}
