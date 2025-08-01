use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

pub trait Movement {
    fn is_moving_left(&self) -> bool;
    fn is_moving_right(&self) -> bool;
    fn is_moving_vertical(&self) -> bool;
}

impl Movement for Velocity {
    fn is_moving_left(&self) -> bool {
        self.linvel.x < -f32::EPSILON
    }

    fn is_moving_right(&self) -> bool {
        self.linvel.x > f32::EPSILON
    }

    fn is_moving_vertical(&self) -> bool {
        self.linvel.y > f32::EPSILON || self.linvel.y < -f32::EPSILON
    }
}

#[derive(Component, Clone, Copy, Default, Debug, Reflect)]
pub struct Life {
    current: u16,
}

const DEFAULT_LIFE: i32 = 5;

impl From<&EntityInstance> for Life {
    fn from(entity_instance: &EntityInstance) -> Self {
        let life = entity_instance
            .get_int_field("life")
            .copied()
            .unwrap_or(DEFAULT_LIFE);
        Life {
            current: life as u16,
        }
    }
}

impl Life {
    pub fn get(&self) -> u16 {
        self.current
    }

    pub fn hit(&mut self, damage: u16) {
        self.current = self.current.saturating_sub(damage);
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    // pub fn add(&mut self, life: u16) {
    //     self.current = std::cmp::min(self.current + life, self.max);
    // }
}

#[derive(Component, Clone, Copy, Default, Debug, Deref, Reflect)]
pub struct Speed(pub f32);

const DEFAULT_SPEED: f32 = 120.;

impl From<&EntityInstance> for Speed {
    fn from(entity_instance: &EntityInstance) -> Self {
        let speed = entity_instance
            .get_float_field("speed")
            .copied()
            .unwrap_or(DEFAULT_SPEED);
        Speed(speed)
    }
}

#[derive(Component, Clone, Copy, Default, Debug, Deref, Reflect)]
pub struct JumpSpeed(pub f32);

const DEFAULT_JUMP_SPEED: f32 = 180.;

impl From<&EntityInstance> for JumpSpeed {
    fn from(entity_instance: &EntityInstance) -> Self {
        let speed = entity_instance
            .get_float_field("jump_speed")
            .copied()
            .unwrap_or(DEFAULT_JUMP_SPEED);
        JumpSpeed(speed)
    }
}

#[derive(Clone, Copy, Component)]
pub struct Damage(pub u16);

#[derive(Component, Debug)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

pub fn ground_sensor(parent: Entity, half_extents: Vec2) -> impl Bundle {
    let pos = Vec3::new(0., -half_extents.y + 1., 0.);

    (
        Name::new("GroundSensor"),
        GroundSensor {
            ground_detection_entity: parent,
            intersecting_ground_entities: HashSet::new(),
        },
        ChildOf(parent),
        Transform::from_translation(pos),
        Collider::cuboid(half_extents.x / 2.0, 1.),
        ActiveEvents::COLLISION_EVENTS,
        Sensor,
    )
}

#[derive(Clone, Default, Component, Debug, Reflect)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component, Default, Clone, Eq, PartialEq, Debug, Reflect)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Component, Clone, Copy)]
#[component(storage = "SparseSet")]
pub struct Dying;

#[derive(Component, Clone, Copy, Default, Reflect, Deref)]
#[component(storage = "SparseSet")]
pub struct InWater(pub bool);

#[derive(Component, Clone, Copy, Default, Reflect, Deref)]
#[component(storage = "SparseSet")]
pub struct Jumping(pub bool);
