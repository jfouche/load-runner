use super::{GameState, InGameState, Temporary};
use crate::ui::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn popup_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_popup, close_popup).run_if(in_state(GameState::InGame)),
    );
}

#[derive(Component, Default)]
pub struct PopupContent {
    pub title: String,
    pub text: String,
    pub images: Vec<PopupImage>,
}

impl PopupContent {
    pub fn add_image(&mut self, image: PopupImage) {
        self.images.push(image);
    }
}

pub enum PopupImage {
    AtlasImage {
        texture_atlas: TextureAtlas,
        image: UiImage,
    },
}

#[derive(Component, Default)]
enum PopupCloseEvent {
    Duration(Duration),
    #[default]
    KeyPressed,
}

#[derive(Bundle)]
pub struct PopupBundle {
    pub content: PopupContent,
    name: Name,
    node: NodeBundle,
    close_event: PopupCloseEvent,
}

impl Default for PopupBundle {
    fn default() -> Self {
        PopupBundle {
            content: PopupContent::default(),
            name: Name::new("Popup"),
            node: centered(),
            close_event: PopupCloseEvent::default(),
        }
    }
}

impl PopupBundle {
    pub fn new(content: PopupContent) -> Self {
        PopupBundle {
            content,
            ..Default::default()
        }
    }

    pub fn auto_despawn(self) -> Self {
        PopupBundle {
            content: self.content,
            name: self.name,
            node: self.node,
            close_event: PopupCloseEvent::Duration(Duration::from_secs_f32(2.0)),
        }
    }
}

fn spawn_popup(
    mut commands: Commands,
    popups: Query<(Entity, &PopupContent, &PopupCloseEvent), Added<PopupContent>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for (entity, content, close_event) in &popups {
        commands.entity(entity).with_children(|screen| {
            screen.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title(&content.title));
                menu.spawn(TextBundle::from_section(
                    content.text.clone(),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
                for image in &content.images {
                    match image {
                        PopupImage::AtlasImage {
                            texture_atlas,
                            image,
                        } => {
                            menu.spawn(AtlasImageBundle {
                                texture_atlas: texture_atlas.clone(),
                                image: image.clone(),
                                ..Default::default()
                            });
                        }
                    }
                }
            });
        });
        if let PopupCloseEvent::Duration(duration) = close_event {
            commands.entity(entity).insert(Temporary::new(*duration));
        }
        in_game_state.set(InGameState::ShowPopup);
    }
}

fn close_popup(
    mut commands: Commands,
    popups: Query<Entity, (With<PopupCloseEvent>, Without<Temporary>)>,
    input: Res<ButtonInput<KeyCode>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if input.get_just_pressed().len() != 0 {
        for entity in &popups {
            commands.entity(entity).despawn_recursive();
            in_game_state.set(InGameState::Running);
        }
    }
}
