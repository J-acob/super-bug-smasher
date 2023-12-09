use crate::state::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetsLoading)
                .continue_to_state(AppState::MainMenu)
                .on_failure_continue_to_state(AppState::Fail),
        )
        .add_collection_to_loading_state::<_, AppAssets>(AppState::AssetsLoading)
        .add_systems(OnEnter(AppState::Fail), fail);
    }
}

#[derive(AssetCollection, Resource)]
pub struct AppAssets {
    #[asset(path = "embedded://fonts/KiwiSoda.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "embedded://sprites/robot1.png")]
    pub robot1_sprite: Handle<Image>,
    #[asset(path = "embedded://sprites/enemy1.png")]
    pub enemy1_sprite: Handle<Image>,
}

fn fail() {
    panic!("Failed to load some asset! :/");
}
