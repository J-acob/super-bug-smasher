use bevy::prelude::*;

use self::prelude::{DetectionPlugin, HealthPlugin};

mod detection;
mod health;

pub mod prelude {
    pub use super::detection::*;
    pub use super::health::*;
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DetectionPlugin, HealthPlugin));
    }
}
