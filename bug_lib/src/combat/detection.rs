use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Hitbox;

#[derive(Component, Default)]
pub struct Hurtbox;

pub struct DetectionPlugin;

impl Plugin for DetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, find_hurtbox_and_hitbox_intersections);
    }
}

/// Locate intersections between hurtboxes and hitboxes
pub fn find_hurtbox_and_hitbox_intersections() {}
