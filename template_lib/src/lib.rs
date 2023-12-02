use bevy::{app::PluginGroupBuilder, prelude::*};
use hello::HelloPlugin;

mod hello;

pub mod prelude {
    pub use bevy::prelude::*;
}
pub struct TemplatePlugins;

impl PluginGroup for TemplatePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Add any of the library plugins here, conditionally if need be.

        group = group.add(HelloPlugin);
        group
    }
}
