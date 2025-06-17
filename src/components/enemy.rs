use crate::components::{
    character::{Damage, Life, Speed},
    GROUP_ENEMY,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
#[require(
    Name::new("Enemy"),
    Life,
    Speed,
    Damage(2),
    Sprite,
    Collider::cuboid(15., 15.),
    RigidBody::KinematicVelocityBased,
    Velocity,
    LockedAxes::ROTATION_LOCKED,
    ActiveEvents::COLLISION_EVENTS,
    CollisionGroups::new(GROUP_ENEMY, Group::ALL)
)]
pub struct Enemy;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
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

#[derive(Clone, Bundle, Default, LdtkEntity)]
pub struct LdtkMobBundle {
    enemy: Enemy,
    #[from_entity_instance]
    life: Life,
    #[from_entity_instance]
    speed: Speed,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[ldtk_entity]
    patrol: Patrol,
}
