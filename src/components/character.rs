use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct Life {
    current: u16,
    // max: u16,
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            current: life,
            // max: life,
        }
    }

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
pub struct Dying;
