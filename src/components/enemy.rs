use crate::in_game::GROUP_ENEMY;

use super::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctly typed");

        for ldtk_point in ldtk_patrol_points {
            // The +1 is necessary here due to the pivot of the entities in the sample
            // file.
            // The patrols set up in the file look flat and grounded,
            // but technically they're not if you consider the pivot,
            // which is at the bottom-center for the skulls.
            let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                * Vec2::splat(layer_instance.grid_size as f32);

            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}

#[derive(Clone, Bundle, LdtkEntity)]
pub struct MobBundle {
    enemy: Enemy,
    name: Name,
    #[from_entity_instance]
    life: Life,
    #[from_entity_instance]
    speed: Speed,
    damage: Damage,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    collider_bundle: ColliderBundle,
    collision_groups: CollisionGroups,
    #[ldtk_entity]
    patrol: Patrol,
}

impl Default for MobBundle {
    fn default() -> Self {
        MobBundle {
            enemy: Enemy,
            name: Name::new("Enemy - Mob"),
            life: Life::default(),
            speed: Speed::default(),
            damage: Damage(2),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(15., 15.),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            collision_groups: CollisionGroups::new(GROUP_ENEMY, Group::ALL),
            patrol: Patrol::default(),
        }
    }
}
