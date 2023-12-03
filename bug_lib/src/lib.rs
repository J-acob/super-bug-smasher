use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use swatter::SwatterPlugin;

mod camera;
mod swatter;

pub mod prelude {
    pub use bevy::prelude::*;
}
pub struct BugGamePlugins;

impl PluginGroup for BugGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Add any of the library plugins here, conditionally if need be.

        group = group
            .add(CameraPlugin)
            .add(SwatterPlugin)
            ;
        group
    }
}
