use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Temporary(Timer);

impl Temporary {
    #[allow(dead_code)]
    pub fn new(duration: Duration) -> Self {
        Temporary(Timer::new(duration, TimerMode::Once))
    }
}

pub fn temporary_plugin(app: &mut App) {
    app.add_systems(Update, remove_entity);
}

fn remove_entity(
    mut commands: Commands,
    mut temps: Query<(Entity, &mut Temporary)>,
    time: Res<Time>,
) {
    for (entity, mut temp) in temps.iter_mut() {
        temp.tick(time.delta());
        if temp.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
