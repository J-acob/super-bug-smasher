use asset_loading::AssetPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use camera::CameraPlugin;
use state::StatePlugin;
use swatter::SwatterPlugin;
use ui::UiPlugin;

mod asset_loading;
mod camera;
mod combat;
mod state;
mod swatter;
mod ui;

pub mod prelude {
    pub use bevy::prelude::*;
}
pub struct BugGamePlugins;

impl PluginGroup for BugGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Add any of the library plugins here, conditionally if need be.

        group = group
            .add(AssetPlugin)
            .add(EmbeddedAssetPlugin::default())
            .add(StatePlugin)
            .add(CameraPlugin)
            .add(SwatterPlugin)
            .add(UiPlugin);
        group
    }
}
