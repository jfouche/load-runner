use super::{GameState, InGameState, Temporary};
use crate::ui::*;
use bevy::prelude::*;

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
    // Duration(Duration),
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
            node: popup(),
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

    // pub fn auto_despawn(self) -> Self {
    //     PopupBundle {
    //         content: self.content,
    //         name: self.name,
    //         node: self.node,
    //         close_event: PopupCloseEvent::Duration(Duration::from_secs_f32(2.0)),
    //     }
    // }
}

fn spawn_popup(
    mut commands: Commands,
    popups: Query<(Entity, &PopupContent, &PopupCloseEvent), Added<PopupContent>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for (entity, content, _close_event) in &popups {
        commands.entity(entity).with_children(|menu| {
            menu.spawn(popup_title_bar()).with_children(|title_bar| {
                title_bar.spawn(popup_title(&content.title));
            });
            menu.spawn(popup_text_content(&content.text));
            for image in &content.images {
                menu.spawn(popup_image(image));
            }
        });
        // if let PopupCloseEvent::Duration(duration) = close_event {
        //     commands.entity(entity).insert(Temporary::new(*duration));
        // }
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

fn popup() -> NodeBundle {
    let vsizer = vsizer();
    NodeBundle {
        background_color: Color::rgb(0.25, 0.25, 0.25).into(),
        border_color: Color::BLACK.into(),
        style: Style {
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(35.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(7.0),
            },
            ..vsizer.style
        },
        ..vsizer
    }
}

fn popup_title_bar() -> NodeBundle {
    NodeBundle {
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        style: Style {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn popup_title(title: &str) -> TextBundle {
    TextBundle::from_section(
        title,
        TextStyle {
            font_size: 32.0,
            color: Color::rgb(0.72, 0.72, 0.72),
            ..default()
        },
    )
}

fn popup_text_content(content: &str) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 24.0,
            color: Color::WHITE,
            ..Default::default()
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(7.0)),
        ..Default::default()
    })
}

fn popup_image(image: &PopupImage) -> impl Bundle {
    match image {
        PopupImage::AtlasImage {
            texture_atlas,
            image,
        } => AtlasImageBundle {
            texture_atlas: texture_atlas.clone(),
            image: image.clone(),
            ..Default::default()
        },
    }
}