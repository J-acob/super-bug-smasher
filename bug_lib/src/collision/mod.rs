use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {

    }
}

/// Collider for collision detection (sorry, only circles because game jam)
#[derive(Component)]
pub struct Collider {
    radius: f32,
}