use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, velocity_moves_transforms);
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    velocity: Velocity,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Speed(f32);

/// Apply velocity to things that want to move.
/// Adapted from https://bevyengine.org/examples/Games/breakout/
pub fn velocity_moves_transforms(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (v, mut t) in query.iter_mut() {
        t.translation.x += v.0.x * time.delta_seconds();
        t.translation.y += v.0.y * time.delta_seconds();
    }
}
