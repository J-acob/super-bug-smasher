use bevy::{prelude::*, time::Stopwatch, transform::commands};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flashers_tick);
    }
}

#[derive(Component)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.)
    }
}

/// Flashes a sprite white and back to it's original color
#[derive(Component)]
pub struct Flasher(pub Timer);

fn flashers_tick(
    mut fq: Query<(Entity, &mut Flasher, &mut Sprite)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (e, mut f, mut s) in fq.iter_mut() {
        f.0.tick(time.delta());

        s.color = Color::rgba(255., 255., 255., 1.);

        if f.0.finished() {
            s.color = Color::WHITE;
            commands.entity(e).remove::<Flasher>();
        }
    }
}
