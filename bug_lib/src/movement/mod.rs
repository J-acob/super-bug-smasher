use bevy::prelude::*;

use crate::state::AppState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            velocity_moves_transforms.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub speed: Speed,
}

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

/// Apply velocity to things that want to move.
/// Adapted from https://bevyengine.org/examples/Games/breakout/
pub fn velocity_moves_transforms(
    mut query: Query<(&Velocity, &Speed, &mut Transform)>,
    time: Res<Time>,
) {
    for (v, s, mut t) in query.iter_mut() {
        t.translation.x += v.0.x * s.0 * time.delta_seconds();
        t.translation.y += v.0.y * s.0 * time.delta_seconds();
    }
}