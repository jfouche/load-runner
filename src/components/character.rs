use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

pub trait Movement {
    fn move_left(&self) -> bool;
    fn move_right(&self) -> bool;
    fn climb(&self) -> bool;
}

impl Movement for Velocity {
    fn move_left(&self) -> bool {
        self.linvel.x < -f32::EPSILON
    }

    fn move_right(&self) -> bool {
        self.linvel.x > f32::EPSILON
    }

    fn climb(&self) -> bool {
        self.linvel.y > f32::EPSILON || self.linvel.y < -f32::EPSILON
    }
}

#[derive(Component, Clone, Copy, Default, Debug, Reflect)]
pub struct Life {
    current: u16,
    // max: u16,
}

impl From<&EntityInstance> for Life {
    fn from(entity_instance: &EntityInstance) -> Self {
        let life = entity_instance
            .get_int_field("life")
            .expect("[life] field should be correctly typed");

        Life {
            current: *life as u16,
        }
    }
}

impl Life {
    pub fn get(&self) -> u16 {
        self.current
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.current {
            self.current = 0;
        } else {
            self.current -= damage;
        }
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

impl From<&EntityInstance> for Speed {
    fn from(entity_instance: &EntityInstance) -> Self {
        let speed = entity_instance
            .get_float_field("speed")
            .expect("[speed] field should be correctly typed");
        warn!(
            "From<&EntityInstance> for Speed : {} = {speed:?}",
            entity_instance.identifier
        );
        Speed(*speed)
    }
}

#[derive(Component, Clone, Copy, Default, Debug, Deref, Reflect)]
pub struct JumpSpeed(pub f32);

impl From<&EntityInstance> for JumpSpeed {
    fn from(entity_instance: &EntityInstance) -> Self {
        let speed = entity_instance
            .get_float_field("jump_speed")
            .expect("[jump_speed] field should be correctly typed");
        warn!(
            "From<&EntityInstance> for JumpSpeed : {} = {speed:?}",
            entity_instance.identifier
        );
        JumpSpeed(*speed)
    }
}

#[derive(Clone, Copy, Component)]
pub struct Damage(pub u16);

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
        let pos = Vec3::new(0., -half_extents.y + 1., 0.);
        GroundSensorCollider {
            name: Name::new("GroundSensor"),
            events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(half_extents.x / 2.0, 1.),
            sensor: Sensor,
            transform: TransformBundle::from_transform(Transform::from_translation(pos)),
            ground_sensor: GroundSensor {
                ground_detection_entity: parent,
                intersecting_ground_entities: HashSet::new(),
            },
        }
    }
}

#[derive(Clone, Default, Component, Debug)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
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
