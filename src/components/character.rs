use bevy::prelude::*;
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

#[derive(Clone, Default, Component)]
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
