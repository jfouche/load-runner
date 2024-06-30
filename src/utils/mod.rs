mod blink;
mod invulnerable;

pub use blink::*;
pub use invulnerable::*;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct UtilsPlugins;

impl PluginGroup for UtilsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BlinkPlugin)
            .add(InvulnerabilityPlugin)
    }
}
