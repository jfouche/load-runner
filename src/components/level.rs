use super::Items;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

/// A simple rectangle type representing a wall of any size
pub struct WallRect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Bundle)]
pub struct WallColliderBundle {
    name: Name,
    collider: Collider,
    body: RigidBody,
    friction: Friction,
    transform: TransformBundle,
}

impl WallColliderBundle {
    pub fn new(wall_rect: WallRect, grid_size: i32) -> Self {
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
            transform: TransformBundle::from_transform(transform),
        }
    }
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

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Bundle)]
pub struct GroundSensorCollider {
    name: Name,
    events: ActiveEvents,
    collider: Collider,
    sensor: Sensor,
    transform: TransformBundle,
    ground_sensor: GroundSensor,
}

impl GroundSensorCollider {
    pub fn new(parent: Entity, half_extents: Vec2) -> Self {
        let pos = Vec3::new(0., -half_extents.y, 0.);
        GroundSensorCollider {
            name: Name::new("GroundSensor"),
            events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(half_extents.x / 2.0, 2.),
            sensor: Sensor,
            transform: TransformBundle::from_transform(Transform::from_translation(pos)),
            ground_sensor: GroundSensor {
                ground_detection_entity: parent,
                intersecting_ground_entities: HashSet::new(),
            },
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
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collider_bundle: ColliderBundle,
}

impl Default for DoorBundle {
    fn default() -> Self {
        DoorBundle {
            tag: Door,
            name: Name::new("Door"),
            expect: Items::default(),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(24., 24.),
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            },
        }
    }
}
