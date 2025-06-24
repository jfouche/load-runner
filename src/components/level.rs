use crate::components::item::Items;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

pub const _ENTITIES_LAYER: usize = 0;
pub const _WALL_SHADOWS_LAYER: usize = 1;
pub const COLLISIONS_LAYER: usize = 2;
pub const _BG_TEXTURES_LAYER: usize = 3;

pub const DIRT_INT_CELL: i32 = 1;
pub const LADDER_INT_CELL: i32 = 2;
pub const STONE_INT_CELL: i32 = 3;
pub const WATER_INT_CELL: i32 = 4;

/// A LDTk cell component that should be use as a collider.
#[derive(Component, Default)]
pub struct ColliderCell;

/// A LDTk cell component that represents dirt, which is destructible.
#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, LdtkIntCell)]
#[require(Name::new("DirtCell"), ColliderCell, Destructible)]
pub struct LdtkDirtCell {}

/// A LDTk cell component that represents stone, which is undestructible.
#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, LdtkIntCell)]
#[require(Name::new("StoneCell"), ColliderCell)]
pub struct LdtkStoneCell {}

/// A LDTk cell component that represents water, which change the local gravity.
#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, LdtkIntCell)]
#[require(Name::new("WaterCell"))]
pub struct LdtkWaterCell {}

/// Marker component that indicate a cell is destructibe
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Destructible;

/// Marker component that indicate a [Destructible] cell is destroyed
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
#[component(storage = "SparseSet")]
pub struct Destroyed;

/// Marker component that indicate a cell is climbable
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

/// A LDTk cell component that represents a ladder, which is [Climbable].
#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, LdtkIntCell)]
#[require(
    Name::new("LadderCell"),
    Climbable,
    Collider::cuboid(8., 8.),
    Sensor,
    LockedAxes::ROTATION_LOCKED,
    ActiveEvents::COLLISION_EVENTS
)]
pub struct LdtkLadderCell {}

#[derive(Component, Clone, Copy, Default)]
#[require(Name::new("Door"), RigidBody::Fixed, Collider::cuboid(8., 16.), Sensor)]
pub struct Door;

#[derive(Bundle, Default, LdtkEntity)]

pub struct LdtkDoorBundle {
    tag: Door,
    #[from_entity_instance]
    expect: Items,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

/// Marker of the end of a level
#[derive(Component, Default)]
#[require(
    Name::new("EndLevel"),
    RigidBody::Fixed,
    Collider::cuboid(8., 16.),
    Sensor
)]
pub struct EndLevel;

#[derive(Bundle, Default, LdtkEntity)]
pub struct LdtkEndLevelBundle {
    tag: EndLevel,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

/// Consider where the walls are
/// storing them as GridCoords in a HashSet for quick, easy lookup
///
/// The key of this map will be the entity of the level the wall belongs to.
/// This has two consequences in the resulting collision entities:
/// 1. it forces the walls to be split along level boundaries
/// 2. it lets us easily add the collision entities as children of the appropriate level entity
pub struct LevelColliders {
    level_to_colliders_locations: HashMap<Entity, HashSet<GridCoords>>,
}

impl LevelColliders {
    pub fn new() -> Self {
        LevelColliders {
            level_to_colliders_locations: HashMap::new(),
        }
    }

    pub fn add_coord(&mut self, level: Entity, coord: GridCoords) {
        self.level_to_colliders_locations
            .entry(level)
            .or_default()
            .insert(coord);
    }

    /// The algorithm used here is a nice compromise between simplicity, speed,
    /// and a small number of rectangle colliders.
    /// In basic terms, it will:
    /// 1. consider where the walls are
    /// 2. combine wall tiles into flat "plates" in each individual row
    /// 3. combine the plates into rectangles across multiple rows wherever possible
    /// It returns all rectangle corresponding to levels colliders
    pub fn rectangles(&self, level: &Entity, width: i32, height: i32) -> Vec<IRect> {
        let mut colliders = vec![];
        if let Some(level_colliders) = self.level_to_colliders_locations.get(level) {
            // combine wall tiles into flat "plates" in each individual row
            let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

            for y in 0..height {
                let mut row_plates: Vec<Plate> = Vec::new();
                let mut plate_start = None;

                // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                for x in 0..width + 1 {
                    match (plate_start, level_colliders.contains(&GridCoords { x, y })) {
                        (Some(s), false) => {
                            row_plates.push(Plate {
                                left: s,
                                right: x - 1,
                            });
                            plate_start = None;
                        }
                        (None, true) => plate_start = Some(x),
                        _ => (),
                    }
                }
                plate_stack.push(row_plates);
            }

            // combine "plates" into rectangles across multiple rows
            let mut rect_builder: HashMap<Plate, IRect> = HashMap::new();
            let mut prev_row: Vec<Plate> = Vec::new();

            // an extra empty row so the algorithm "finishes" the rects that touch the top edge
            plate_stack.push(Vec::new());

            for (y, current_row) in plate_stack.into_iter().enumerate() {
                for prev_plate in &prev_row {
                    if !current_row.contains(prev_plate) {
                        // remove the finished rect so that the same plate in the future starts a new rect
                        if let Some(rect) = rect_builder.remove(prev_plate) {
                            // colliders.push(WallColliderBundle::new(rect, grid_size));
                            colliders.push(rect);
                        }
                    }
                }
                for plate in &current_row {
                    rect_builder
                        .entry(plate.clone())
                        .and_modify(|rect| rect.max.y += 1)
                        .or_insert(IRect::new(plate.left, y as i32, plate.right, y as i32));
                }
                prev_row = current_row;
            }
        }
        colliders
    }
}

#[derive(Component)]
pub struct LevelCollider;

pub fn level_collider(rect: IRect, grid_size: i32) -> impl Bundle {
    let scale = grid_size as f32 / 2.;
    let half_size = (rect.size() + ivec2(1, 1)).as_vec2() * scale;
    let pos = ivec2(rect.min.x + rect.max.x + 1, rect.min.y + rect.max.y + 1).as_vec2() * scale;
    (
        LevelCollider,
        Name::new("WallCollider"),
        Transform::from_translation(pos.extend(0.)),
        Collider::cuboid(half_size.x, half_size.y),
        RigidBody::Fixed,
        Friction::new(1.0),
    )
}

/// Represents a wide wall that is 1 tile tall
/// Used to spawn wall collisions
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Plate {
    left: i32,
    right: i32,
}

#[derive(Event)]
pub struct UpdateCollidersEvent;
