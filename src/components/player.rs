use super::*;
use crate::in_game::GROUP_PLAYER;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    name: Name,
    life: Life,
    #[sprite_bundle("player/walk.png")]
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    collider_bundle: ColliderBundle,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    #[worldly]
    worldly: Worldly,
    climber: Climber,
    ground_detection: GroundDetection,
    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Items,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            name: Name::new("Player"),
            life: Life::new(10),
            sprite_bundle: SpriteBundle::default(),
            texture_atlas: TextureAtlas::default(),
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(7., 8.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            collision_groups: CollisionGroups::new(GROUP_PLAYER, Group::ALL),
            active_events: ActiveEvents::COLLISION_EVENTS,
            worldly: Worldly::default(),
            climber: Climber::default(),
            ground_detection: GroundDetection::default(),
            items: Items::default(),
            entity_instance: EntityInstance::default(),
        }
    }
}

#[derive(Event)]
pub struct PlayerDiedEvent;
