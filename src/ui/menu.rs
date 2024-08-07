use bevy::{color::palettes::css::*, prelude::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BUTTON_TEXT_SIZE: f32 = 24.0;

pub fn button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: button_style(),
        background_color: NORMAL_BUTTON.into(),
        border_color: Color::BLACK.into(),
        ..default()
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(160.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

pub fn button_text_style() -> TextStyle {
    TextStyle {
        font_size: BUTTON_TEXT_SIZE,
        color: BUTTON_TEXT_COLOR,
        ..default()
    }
}

pub fn button_text(text: &str) -> TextBundle {
    TextBundle::from_section(text, button_text_style())
}

pub fn menu() -> NodeBundle {
    let vsizer = vsizer();
    NodeBundle {
        background_color: CRIMSON.into(),
        border_color: BLUE.into(),
        style: Style {
            border: UiRect::all(Val::Px(2.0)),
            ..vsizer.style
        },
        ..vsizer
    }
}

pub fn menu_title(title: &str) -> TextBundle {
    TextBundle::from_section(
        title,
        TextStyle {
            font_size: 60.0,
            color: BUTTON_TEXT_COLOR,
            ..default()
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
    })
}

pub fn hsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

pub fn vsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

/// This system updates the settings when a new value for a setting is selected, and marks
/// the button as the one currently selected
#[allow(dead_code)]
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), (With<SelectedOption>, With<T>)>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            if let Ok((previous_button, mut previous_color)) = selected_query.get_single_mut() {
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
            }
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
