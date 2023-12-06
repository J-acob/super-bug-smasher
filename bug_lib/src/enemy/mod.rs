use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{combat::prelude::Health, state::AppState, movement::{Velocity, self, MovementBundle}, tower::Tower};
use rand::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(EnemySpawnConfig {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            spawn_radius: 500.,
        })
        .add_systems(Update, (debug_enemies, enemies_spawn, enemies_hate_the_tower).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    transform: Transform,
    health: Health,
    movement_bundle: MovementBundle,
    marker: Enemy,
}

#[derive(Resource)]
pub struct EnemySpawnConfig {
    pub timer: Timer,
    pub spawn_radius: f32,
}

fn enemies_spawn(mut commands: Commands, time: Res<Time>, mut config: ResMut<EnemySpawnConfig>) {
    config.timer.tick(time.delta());

    // Get a random point on the edge of the circle

    if config.timer.finished() {
        
        let mut rng = rand::thread_rng();
        let random_angle: f32 = rng.gen_range(0.0..=1000.) * PI * 2.;
        //let random_x: f32 = rng.gen_range(0.0..100.);
        //let random_y: f32 = rng.gen_range(0.0..100.);
        //let x = 0. + config.spawn_radius + (random_angle * 3.14 / 180.).cos();
        //let y = 0. + config.spawn_radius + (random_angle * 3.14 / 180.).sin();
        let x = random_angle.cos() * config.spawn_radius;
        let y = random_angle.sin() * config.spawn_radius;
        
        commands.spawn(
            EnemyBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            }
        );
    }
}

fn debug_enemies(q: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (e, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), 16., Color::RED);
    }
}

fn enemies_hate_the_tower(mut enemy_q: Query<(&Enemy, &Transform, &mut Velocity)>, tower_q: Query<(&Tower, &Transform)>) {
    let (_, tower_transform) = tower_q.single();

    for (_, enemy_transform, mut velocity) in enemy_q.iter_mut() {
        let movement_vector = tower_transform.translation.xy() - enemy_transform.translation.xy();
        velocity.0 = movement_vector;
    }
}