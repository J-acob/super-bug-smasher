use asset_loading::AssetPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use state::StatePlugin;
use swatter::SwatterPlugin;
use tower::TowerPlugin;
use ui::UiPlugin;

mod asset_loading;
mod camera;
mod collision;
mod combat;
mod enemy;
mod movement;
mod state;
mod steering;
mod swatter;
mod tower;
mod ui;
mod audio;

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
            .add(UiPlugin)
            .add(EnemyPlugin)
            .add(MovementPlugin)
            .add(CollisionPlugin)
            .add(TowerPlugin);
        group
    }
}
