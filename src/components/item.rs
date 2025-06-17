use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::slice::Iter;

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

/// The items component is used to store the items that a chest contains.
#[derive(Component, Copy, Clone, Debug, Eq, Default, PartialEq)]
#[require(Name::new("Chest"), Items, RigidBody::Fixed, Collider::cuboid(8., 8.))]
pub struct Chest;

#[derive(Clone, Bundle, Default, LdtkEntity)]
pub struct LdtkChestBundle {
    tag: Chest,
    #[from_entity_instance]
    items: Items,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Resource, Clone, Asset, TypePath)]
pub struct ItemAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        ItemAssets {
            texture: world.load_asset("atlas/MV Icons Complete Sheet Free - ALL.png"),
            texture_atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::new(32, 32),
                16,
                95,
                None,
                None,
            )),
        }
    }
}

impl ItemAssets {
    pub fn image_node(&self, item: Item) -> ImageNode {
        let index = match item {
            Item::Boots => 18,
            Item::Key => 83,
            Item::Gem => 1483,
            Item::Unknown => 0,
        };
        ImageNode::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }
}
