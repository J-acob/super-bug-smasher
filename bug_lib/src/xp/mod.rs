use bevy::prelude::*;

use crate::collision::Collider;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Default)]
pub struct Experience;

#[derive(Bundle, Default)]
pub struct ExperienceBundle {
    pub collider: Collider,
    pub sprite_bundle: SpriteBundle,
    pub marker: Experience,
}
