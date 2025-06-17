use crate::components::{
    character::{
        AnimationTimer, Climber, GroundDetection, InWater, JumpSpeed, Jumping, Life, Speed,
    },
    item::Items,
    GROUP_PLAYER,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

/// All [Player] assets
#[derive(Resource, Clone, Asset, TypePath)]
pub struct PlayerAssets {
    pub walk_sprites: Handle<Image>,
    pub walk_atlas_layout: Handle<TextureAtlasLayout>,
    pub jump_sprites: Handle<Image>,
    pub jump_atlas_layout: Handle<TextureAtlasLayout>,
    pub death_sprites: Handle<Image>,
    pub death_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        PlayerAssets {
            walk_sprites: world.load_asset("player/walk.png"),
            walk_atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::splat(16),
                8,
                4,
                Some(UVec2::splat(64)),
                Some(UVec2::splat(32)),
            )),
            jump_sprites: world.load_asset("player/jump.png"),
            jump_atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::splat(16),
                6,
                4,
                Some(UVec2::splat(64)),
                Some(UVec2::splat(32)),
            )),
            death_sprites: world.load_asset("player/death.png"),
            death_atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::splat(16),
                6,
                4,
                Some(UVec2::splat(64)),
                Some(UVec2::splat(32)),
            )),
        }
    }
}

/// The player component
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
#[require(
    Name::new("Player"),
    // Caracteristics
    Life,
    Speed,
    JumpSpeed,
    Items,
    // Markers
    Climber,
    GroundDetection,
    Jumping,
    InWater,
    // Sprite
    Sprite,
    AnimationTimer,
    // Physics
    Collider::round_cuboid(3., 5., 2.),
    RigidBody::Dynamic,
    Velocity,
    LockedAxes::ROTATION_LOCKED,
    GravityScale,
    Friction {
        coefficient: 0.0,
        combine_rule: CoefficientCombineRule::Min,
    },
    ColliderMassProperties,
    CollisionGroups::new(GROUP_PLAYER, Group::ALL),
    ActiveEvents::COLLISION_EVENTS,
)]
pub struct Player;

/// LDTk adaptor for [Player]
#[derive(Bundle, Default, LdtkEntity)]
pub struct LdtkPlayerBundle {
    player: Player,
    #[from_entity_instance]
    life: Life,
    #[from_entity_instance]
    speed: Speed,
    #[from_entity_instance]
    jump_speed: JumpSpeed,
    #[sprite("player/walk.png")]
    sprite: Sprite,
    #[worldly]
    worldly: Worldly,
    #[from_entity_instance]
    items: Items,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Event)]
pub struct PlayerDeathEvent;

#[derive(Event)]
pub struct DigEvent;
