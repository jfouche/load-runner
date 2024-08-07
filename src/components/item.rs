use std::slice::Iter;

use super::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum Item {
    Gem,
    Boots,
    Key,
    Unknown,
}

impl From<&String> for Item {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "Gem" => Item::Gem,
            "Boots" => Item::Boots,
            "Key" => Item::Key,
            _ => {
                error!("Unknown item {value}");
                Item::Unknown
            }
        }
    }
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq, Reflect)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn iter(&self) -> Iter<Item> {
        self.0.iter()
    }

    pub fn add(&mut self, item: Item) {
        self.0.push(item);
    }

    pub fn contains(&self, item: Item) -> bool {
        self.0.contains(&item)
    }

    pub fn contains_items(&self, items: &Items) -> bool {
        for item in items.0.iter() {
            if !self.0.contains(item) {
                return false;
            }
        }
        true
    }

    pub fn remove_items(&mut self, items: &Items) {
        for item in items.0.iter() {
            if let Some(idx) = self.0.iter().position(|i| item == i) {
                self.0.remove(idx);
            }
        }
    }
}

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
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    collider_bundle: ColliderBundle,
}

impl Default for ChestBundle {
    fn default() -> Self {
        ChestBundle {
            tag: Chest,
            name: Name::new("Chest"),
            items: Items::default(),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
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
    pub fn image_components(&self, item: Item) -> (UiImage, TextureAtlas) {
        let index = match item {
            Item::Boots => 18,
            Item::Key => 83,
            Item::Gem => 1483,
            Item::Unknown => 0,
        };
        (
            UiImage::new(self.texture.clone()),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }

    pub fn image_bundle(&self, item: Item) -> (ImageBundle, TextureAtlas) {
        let components = self.image_components(item);
        (
            ImageBundle {
                image: components.0,
                ..Default::default()
            },
            components.1,
        )
    }
}
