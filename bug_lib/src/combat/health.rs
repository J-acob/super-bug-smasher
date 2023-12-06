use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.)
    }
}
