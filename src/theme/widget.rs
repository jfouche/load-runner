//! Helper functions for creating common widgets.

use crate::theme::palette::*;
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};
use std::borrow::Cow;

pub fn hsizer() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn vsizer() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn centered() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple popup
pub fn popup() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(40.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::bottom(Val::Px(7.0)),
            ..Default::default()
        },
        BackgroundColor(POPUP_BACKGROUND),
        BorderColor(Color::BLACK),
        Pickable::IGNORE,
    )
}

/// The title of a [popup]
pub fn popup_title(title: impl Into<String>) -> impl Bundle {
    (
        Name::new("PopupTitle"),
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        children![
            Text(title.into()),
            TextFont::from_font_size(32.),
            TextColor(Color::srgb(0.72, 0.72, 0.72))
        ],
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(48.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn menu_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        MENU_BUTTON_PALETTE,
        action,
        (
            Node {
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

// /// A small square button with text and an action defined as an [`Observer`].
// pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
// where
//     E: Event,
//     B: Bundle,
//     I: IntoObserverSystem<E, B, M>,
// {
//     button_base(
//         text,
//         action,
//         Node {
//             width: Val::Px(30.0),
//             height: Val::Px(30.0),
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             ..default()
//         },
//     )
// }

/// A simple button with text and an action defined as an [`Observer`].
/// The button's layout is provided by `button_bundle`.
pub fn button_base<E, B, M, I>(
    text: impl Into<String>,
    palette: ButtonPalette,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    BackgroundColor(palette.interaction.none),
                    palette.interaction,
                    children![(
                        Text(text),
                        TextFont::from_font_size(palette.text_size),
                        TextColor(palette.text_color),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}
