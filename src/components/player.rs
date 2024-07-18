use super::*;
use crate::in_game::GROUP_PLAYER;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct PlayerAssets {
    #[allow(dead_code)]
    pub walk_sprites: Handle<Image>,
    #[allow(dead_code)]
    pub walk_atlas_layout: Handle<TextureAtlasLayout>,
    pub death_sprites: Handle<Image>,
    pub death_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    name: Name,
    #[from_entity_instance]
    life: Life,
    #[from_entity_instance]
    speed: Speed,
    #[from_entity_instance]
    jump_speed: JumpSpeed,
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
    in_water: InWater,
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
            life: Life::default(),
            speed: Speed::default(),
            jump_speed: JumpSpeed::default(),
            sprite_bundle: SpriteBundle::default(),
            texture_atlas: TextureAtlas::default(),
            animation_timer: AnimationTimer::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::round_cuboid(3., 5., 2.),
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
            in_water: InWater(false),
            items: Items::default(),
            entity_instance: EntityInstance::default(),
        }
    }
}

#[derive(Event)]
pub struct PlayerDeathEvent;
