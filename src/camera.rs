use crate::{components::*, schedule::InGameSet};
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            camera_follow_player.run_if(in_state(InGameState::LoadLevel)),
        )
        .add_systems(
            Update,
            (camera_follow_player).in_set(InGameSet::EntityUpdate),
        );
}

// const ASPECT_RATIO: f32 = 16. / 9.;
const CAM_LERP_FACTOR: f32 = 2.;

fn spawn_camera(mut commands: Commands) {
    let camera = Camera {
        hdr: true, // HDR is required for the bloom effect
        ..default()
    };
    let projection = OrthographicProjection {
        near: -1000.0,
        far: 1000.0,
        scaling_mode: bevy::render::camera::ScalingMode::WindowSize(2.0),
        ..Default::default()
    };
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera,
            projection,
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}

// fn camera_fit_inside_current_level(
//     mut camera_query: Query<
//         (
//             &mut bevy::render::camera::OrthographicProjection,
//             &mut Transform,
//         ),
//         Without<Player>,
//     >,
//     player_query: Query<&Transform, With<Player>>,
//     level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
//     ldtk_projects: Query<&Handle<LdtkProject>>,
//     level_selection: Res<LevelSelection>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
// ) {
//     let Ok(Transform {
//         translation: player_translation,
//         ..
//     }) = player_query.get_single()
//     else {
//         return;
//     };

//     let ldtk_project = ldtk_project_assets
//         .get(ldtk_projects.single())
//         .expect("Project should be loaded if level has spawned");

//     let Some((level_transform, level)) =
//         level_query.iter().find_map(|(level_transform, level_iid)| {
//             let level = ldtk_project.get_raw_level_by_iid(&level_iid.to_string())?;
//             level_selection
//                 .is_match(&LevelIndices::default(), level)
//                 .then_some((level_transform, level))
//         })
//     else {
//         return;
//     };

//     let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
//     let level_ratio = level.px_wid as f32 / level.px_hei as f32;
//     orthographic_projection.viewport_origin = Vec2::ZERO;
//     if level_ratio > ASPECT_RATIO {
//         // level is wider than the screen
//         let height = (level.px_hei as f32 / 9.).round() * 9.;
//         let width = height * ASPECT_RATIO;
//         orthographic_projection.scaling_mode =
//             bevy::render::camera::ScalingMode::Fixed { width, height };
//         camera_transform.translation.x =
//             (player_translation.x - level_transform.translation.x - width / 2.)
//                 .clamp(0., level.px_wid as f32 - width);
//         camera_transform.translation.y = 0.;
//     } else {
//         // level is taller than the screen
//         let width = (level.px_wid as f32 / 16.).round() * 16.;
//         let height = width / ASPECT_RATIO;
//         orthographic_projection.scaling_mode =
//             bevy::render::camera::ScalingMode::Fixed { width, height };
//         camera_transform.translation.y =
//             (player_translation.y - level_transform.translation.y - height / 2.)
//                 .clamp(0., level.px_hei as f32 - height);
//         camera_transform.translation.x = 0.;
//     }

//     camera_transform.translation.x += level_transform.translation.x;
//     camera_transform.translation.y += level_transform.translation.y;
// }

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}
