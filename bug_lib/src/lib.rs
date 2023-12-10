use asset_loading::AssetPlugin;
use audio::AudioPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use combat::CombatPlugin;
use enemy::EnemyPlugin;
use game::GamePlugin;
use movement::MovementPlugin;
use projectile::ProjectilePlugin;
use state::StatePlugin;
use swatter::SwatterPlugin;
use tower::TowerPlugin;
use ui::UiPlugin;

mod asset_loading;
mod audio;
mod camera;
mod collision;
mod combat;
mod enemy;
mod game;
mod movement;
mod projectile;
mod state;
mod steering;
mod swatter;
mod tower;
mod ui;
mod xp;

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
            .add(TowerPlugin)
            .add(GamePlugin)
            .add(CombatPlugin)
            .add(AudioPlugin)
            .add(ProjectilePlugin);
        group
    }
}
