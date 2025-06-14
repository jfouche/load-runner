use crate::{
    components::player::Player,
    schedule::{InGameSet, InGameState},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (camera_fit_inside_current_level).run_if(in_state(InGameState::LevelLoaded)),
        )
        .add_systems(
            Update,
            (camera_fit_inside_current_level).in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (camera_fit_inside_current_level).in_set(InGameSet::EntityUpdate),
        );
}

const ASPECT_RATIO: f32 = 16. / 9.;
// const CAM_LERP_FACTOR: f32 = 2.;

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection::default_2d();
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic(projection),
    ));
}

fn camera_fit_inside_current_level(
    mut cameras: Query<(&mut Projection, &mut Transform), (With<Camera2d>, Without<Player>)>,
    players: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<Camera2d>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    let player_transform = players.single()?;
    let player_translation = player_transform.translation;

    let (mut projection, mut camera_transform) = cameras.single_mut()?;
    let Projection::Orthographic(ref mut orthographic_projection) = &mut *projection else {
        return Err(BevyError::from("non-orthographic projection found"));
    };
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single()?)
        .expect("Project should be loaded if level has spawned");

    for (level_transform, level_iid) in &level_query {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");
        if level_selection.is_match(&LevelIndices::default(), level) {
            let level_ratio = level.px_wid as f32 / level.px_hei as f32;
            orthographic_projection.viewport_origin = Vec2::ZERO;
            if level_ratio > ASPECT_RATIO {
                // level is wider than the screen
                let height = (level.px_hei as f32 / 9.).round() * 9.;
                let width = height * ASPECT_RATIO;
                orthographic_projection.scaling_mode =
                    bevy::render::camera::ScalingMode::Fixed { width, height };
                camera_transform.translation.x =
                    (player_translation.x - level_transform.translation.x - width / 2.)
                        .clamp(0., level.px_wid as f32 - width);
                camera_transform.translation.y = 0.;
            } else {
                // level is taller than the screen
                let width = (level.px_wid as f32 / 16.).round() * 16.;
                let height = width / ASPECT_RATIO;
                orthographic_projection.scaling_mode =
                    bevy::render::camera::ScalingMode::Fixed { width, height };
                camera_transform.translation.y =
                    (player_translation.y - level_transform.translation.y - height / 2.)
                        .clamp(0., level.px_hei as f32 - height);
                camera_transform.translation.x = 0.;
            }

            camera_transform.translation.x += level_transform.translation.x;
            camera_transform.translation.y += level_transform.translation.y;
        }
    }
    Ok(())
}
