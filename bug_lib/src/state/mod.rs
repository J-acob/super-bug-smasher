use bevy::prelude::*;

pub struct StatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    AssetsLoading,
    MainMenu,
    InGame,
    GameOver,
    Paused,
    Fail,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}
