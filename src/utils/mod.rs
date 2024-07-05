mod blink;
mod invulnerable;
mod temp;

pub use blink::*;
pub use invulnerable::*;
pub use temp::Temporary;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct UtilsPlugins;

impl PluginGroup for UtilsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BlinkPlugin)
            .add(InvulnerabilityPlugin)
            .add(temp::temporary_plugin)
    }
}
