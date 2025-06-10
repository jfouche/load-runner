use crate::{theme::widget, utils::despawn_after::DespawnAfter};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

pub fn popup_with_images_plugin(app: &mut App) {
    app.add_systems(Update, close_popup);
}

#[derive(Component, Default)]
pub struct PopupWithImages;

pub fn popup_with_images(
    title: impl Into<String>,
    text: impl Into<String>,
    images: Vec<ImageNode>,
) -> impl Bundle {
    (
        PopupWithImages,
        Name::new("Popup"),
        widget::popup(),
        children![
            // TITLE
            widget::popup_title(title.into()),
            // TEXT
            (
                Text(text.into()),
                TextFont::from_font_size(24.),
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(7.0)),
                    ..Default::default()
                }
            ),
            // IMAGES
            (
                widget::hsizer(),
                Children::spawn(SpawnWith(move |p: &mut ChildSpawner| {
                    for img in images {
                        p.spawn(img);
                    }
                }))
            )
        ],
    )
}

fn close_popup(
    mut commands: Commands,
    popups: Query<Entity, (With<PopupWithImages>, Without<DespawnAfter>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.get_just_pressed().len() != 0 {
        for entity in &popups {
            commands.entity(entity).despawn();
        }
    }
}
