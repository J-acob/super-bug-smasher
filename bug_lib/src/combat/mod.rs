use bevy::{prelude::*, transform::commands};

use self::prelude::{Health, HealthPlugin};

mod health;

pub mod prelude {
    pub use super::health::*;
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HealthPlugin))
            .add_systems(Update, read_damage_events)
            .add_event::<DamageEvent>();
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub amount: f32,
    pub target: Entity,
}

pub fn read_damage_events(mut evr: EventReader<DamageEvent>, mut hq: Query<&mut Health>) {
    for e in evr.read() {
        let Ok(mut hp) = hq.get_component_mut::<Health>(e.target) else {
            return;
        };
        hp.0 -= e.amount;
    }
}
