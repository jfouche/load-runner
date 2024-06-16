use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn character_plugin(app: &mut App) {
    app.add_systems(Update, (detect_climb_range, ignore_gravity_if_climbing));
}

fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.insert(climbable);
                }
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.insert(climbable);
                };
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }

                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }
            }
        }
    }
}

fn ignore_gravity_if_climbing(mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}
