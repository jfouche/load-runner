mod fade;
// mod menu;
mod progressbar;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use fade::*;
// pub use menu::*;
pub use progressbar::*;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(progressbar::plugin)
            .add(fade::plugin)
    }
}

pub fn fullscreen_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
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
