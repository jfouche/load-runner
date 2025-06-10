use super::blink::Blink;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};
use std::time::Duration;

pub struct InvulnerabilityPlugin;

impl Plugin for InvulnerabilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (invulnerability_started, invulnerability_finished));
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Invulnerable {
    filters: Group,
    timer: Timer,
    pause: bool,
}

impl Invulnerable {
    pub fn new(duration: Duration, filters: Group) -> Self {
        Invulnerable {
            timer: Timer::new(duration, TimerMode::Once),
            filters,
            pause: false,
        }
    }

    /// pause invulnerability
    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }
}

///
/// [`Invulnerable`] stard
///
fn invulnerability_started(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CollisionGroups, &Invulnerable), Added<Invulnerable>>,
) {
    for (entity, mut collision_groups, invulnerable) in &mut query {
        info!("invulnerability_started");
        commands
            .entity(entity)
            .insert(Blink::new(Duration::from_secs_f32(0.15)));
        collision_groups.filters &= !invulnerable.filters;
    }
}

///
/// [`Invulnerable`] finishes
///
fn invulnerability_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CollisionGroups, &mut Invulnerable)>,
) {
    for (entity, mut collision_groups, mut invulnerable) in &mut query {
        if !invulnerable.pause {
            invulnerable.timer.tick(time.delta());
            if invulnerable.timer.just_finished() {
                info!("invulnerability_finished");
                collision_groups.filters |= invulnerable.filters;
                commands.entity(entity).remove::<(Invulnerable, Blink)>();
            }
        }
    }
}
