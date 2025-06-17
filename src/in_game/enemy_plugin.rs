use crate::{
    components::{character::Speed, enemy::Patrol},
    schedule::InGameSet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn enemy_plugin(app: &mut App) {
    app.register_type::<Patrol>()
        .add_systems(Update, patrol.in_set(InGameSet::EntityUpdate));
}

fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &Speed, &mut Patrol)>) {
    for (mut transform, mut velocity, &speed, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * *speed;

        if new_velocity.dot(velocity.linvel) < 0. {
            if patrol.index == 0 {
                patrol.forward = true;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity = (patrol.points[patrol.index] - transform.translation.truncate())
                .normalize()
                * *speed;
        }

        velocity.linvel = new_velocity;
    }
}
