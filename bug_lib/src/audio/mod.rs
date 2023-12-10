use bevy::{audio::PlaybackMode, prelude::*};

use crate::{asset_loading::AppAssets, state::AppState};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            in_game_audio,
        )
        .add_systems(
            OnTransition {
                from: AppState::InGame,
                to: AppState::GameOver,
            },
            (despawn_audio, apply_deferred, game_over_audio).chain(),
        )
        .add_systems(
            OnTransition {
                from: AppState::GameOver,
                to: AppState::MainMenu,
            },
            despawn_audio,
        );
    }
}

/// Marker component for the current audio state
#[derive(Component)]
pub struct CurrentStateAudio;

pub fn in_game_audio(mut commands: Commands, assets: Res<AppAssets>) {
    commands.spawn((
        AudioBundle {
            source: assets.in_game_audio.clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..Default::default()
            },
            ..Default::default()
        },
        CurrentStateAudio,
    ));
}

pub fn game_over_audio(mut commands: Commands, assets: Res<AppAssets>) {
    commands.spawn((
        AudioBundle {
            source: assets.game_over_audio.clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..Default::default()
            },
            ..Default::default()
        },
        CurrentStateAudio,
    ));
}

pub fn despawn_audio(mut commands: Commands, q: Query<(Entity, &CurrentStateAudio)>) {
    let (e, _) = q.single();
    commands.entity(e).despawn_recursive();
}
