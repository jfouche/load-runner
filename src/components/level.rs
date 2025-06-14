use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::components::item::Items;

pub const _ENTITIES_LAYER: usize = 0;
pub const _WALL_SHADOWS_LAYER: usize = 1;
pub const COLLISIONS_LAYER: usize = 2;
pub const _BG_TEXTURES_LAYER: usize = 3;

pub const DIRT_INT_CELL: i32 = 1;
pub const LADDER_INT_CELL: i32 = 2;
pub const STONE_INT_CELL: i32 = 3;
pub const WATER_INT_CELL: i32 = 4;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    tag: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Water;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WaterBundle {
    tag: Water,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    name: Name,
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub climbable: Climbable,
}

impl Default for LadderBundle {
    fn default() -> Self {
        LadderBundle {
            name: Name::new("Ladder"),
            sensor_bundle: SensorBundle::default(),
            climbable: Climbable,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: Collider::cuboid(8., 8.),
                sensor: Sensor,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            SensorBundle::default()
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Door;

#[derive(Bundle, LdtkEntity)]

pub struct DoorBundle {
    tag: Door,
    name: Name,
    #[from_entity_instance]
    expect: Items,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    collider_bundle: ColliderBundle,
}

impl Default for DoorBundle {
    fn default() -> Self {
        DoorBundle {
            tag: Door,
            name: Name::new("Door"),
            expect: Items::default(),
            sprite_sheet: Sprite::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(8., 16.),
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct EndLevel;

#[derive(Bundle, LdtkEntity)]
pub struct EndLevelBundle {
    tag: EndLevel,
    name: Name,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    collider_bundle: ColliderBundle,
    sensor: Sensor,
}

impl Default for EndLevelBundle {
    fn default() -> Self {
        EndLevelBundle {
            tag: EndLevel,
            name: Name::new("EndLevel"),
            sprite_sheet: Sprite::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(8., 16.),
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            },
            sensor: Sensor,
        }
    }
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
    /// 4. spawn colliders for each rectangle
    pub fn combine(
        &self,
        level: &Entity,
        width: i32,
        height: i32,
        grid_size: i32,
    ) -> Vec<WallColliderBundle> {
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
            let mut rect_builder: HashMap<Plate, WallRect> = HashMap::new();
            let mut prev_row: Vec<Plate> = Vec::new();

            // an extra empty row so the algorithm "finishes" the rects that touch the top edge
            plate_stack.push(Vec::new());

            for (y, current_row) in plate_stack.into_iter().enumerate() {
                for prev_plate in &prev_row {
                    if !current_row.contains(prev_plate) {
                        // remove the finished rect so that the same plate in the future starts a new rect
                        if let Some(rect) = rect_builder.remove(prev_plate) {
                            colliders.push(WallColliderBundle::new(rect, grid_size));
                        }
                    }
                }
                for plate in &current_row {
                    rect_builder
                        .entry(plate.clone())
                        .and_modify(|wr| wr.top += 1)
                        .or_insert(WallRect {
                            bottom: y as i32,
                            top: y as i32,
                            left: plate.left,
                            right: plate.right,
                        });
                }
                prev_row = current_row;
            }
        }
        colliders
    }
}

#[derive(Bundle)]
pub struct WallColliderBundle {
    name: Name,
    collider: Collider,
    body: RigidBody,
    friction: Friction,
    transform: Transform,
}

impl WallColliderBundle {
    fn new(wall_rect: WallRect, grid_size: i32) -> Self {
        let collider = Collider::cuboid(
            (wall_rect.right as f32 - wall_rect.left as f32 + 1.) * grid_size as f32 / 2.,
            (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.) * grid_size as f32 / 2.,
        );
        let transform = Transform::from_xyz(
            (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.,
            (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.,
            0.,
        );
        WallColliderBundle {
            name: Name::new("WallCollider"),
            collider,
            body: RigidBody::Fixed,
            friction: Friction::new(1.0),
            transform,
        }
    }
}

/// A simple rectangle type representing a wall of any size
struct WallRect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

/// Represents a wide wall that is 1 tile tall
/// Used to spawn wall collisions
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Plate {
    left: i32,
    right: i32,
}
