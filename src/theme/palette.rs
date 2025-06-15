use crate::theme::interaction::InteractionPalette;
use bevy::prelude::*;

pub struct ButtonPalette {
    pub interaction: InteractionPalette,
    pub text_color: Color,
    pub text_size: f32,
}

/// Main menu
pub const MAIN_MENU_BACKGROUND: Color = Color::srgb_u8(27, 35, 46);

/// Menu button palette
pub const MENU_BUTTON_PALETTE: ButtonPalette = ButtonPalette {
    interaction: InteractionPalette {
        none: Color::srgb_u8(138, 181, 184),
        hovered: Color::srgb_u8(101, 133, 135),
        pressed: Color::srgb_u8(82, 108, 110),
    },
    text_color: Color::srgb_u8(11, 15, 15),
    text_size: 24.,
};

pub const MENU_BUTTON_TEXT_COLOR: Color = Color::srgb_u8(11, 15, 15);

/// #ddd369
pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

/// #fcfbcc
pub const HEADER_TEXT: Color = Color::srgb(0.2, 0.0, 0.0);

/// Button text color
pub const BUTTON_TEXT: Color = Color::srgb(0.56, 0.56, 0.86);
/// Button background color
pub const BUTTON_BACKGROUND: Color = Color::srgb_u8(138, 181, 184);
/// #6299d1
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.384, 0.600, 0.820);
/// #3d4999
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.239, 0.286, 0.600);

pub const POPUP_BACKGROUND: Color = Color::srgb(0.25, 0.25, 0.25);
pub const POPUP_BORDER: Color = Color::BLACK;
