use bevy::prelude::*;

use crate::{collision::Collider, combat::prelude::Health, state::AppState};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            setup,
        )
        .add_systems(Update, debug_tower);
    }
}

#[derive(Component, Default)]
pub struct Tower;

#[derive(Bundle, Default)]
pub struct TowerBundle {
    marker: Tower,
    health: Health,
    collider: Collider,
    transform: Transform,
}

/// Spawns the tower
fn setup(mut commands: Commands) {
    commands.spawn(TowerBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        collider: Collider { radius: 16. },
        ..default()
    });
}

/// Shows a thingy to represent the tower
fn debug_tower(q: Query<(&Tower, &Transform)>, mut gizmos: Gizmos) {
    for (_, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), 16., Color::GREEN);
    }
}
