use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Update, patrol);
}

fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;

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

            new_velocity =
                (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;
        }

        velocity.linvel = new_velocity;
    }
}
