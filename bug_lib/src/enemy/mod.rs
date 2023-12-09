use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::Texture};

use crate::{
    collision::Collider,
    combat::prelude::Health,
    movement::{self, velocity_moves_transforms, MovementBundle, Speed, Velocity},
    state::AppState,
    tower::Tower, asset_loading::AppAssets,
};
use rand::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnConfig {
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
            spawn_radius: Vec2::new(1920. / 1.5, 1080. / 1.5),
        })
        .add_systems(
            Update,
            ((
                enemies_spawn,
                enemies_hate_the_tower.before(velocity_moves_transforms),
                debug_enemies.after(velocity_moves_transforms),
            )
                .chain()
                .distributive_run_if(in_state(AppState::InGame)))
            .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (enemies_damage_the_tower, flip_enemy_sprite_with_velocity).distributive_run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    //transform: Transform,
    health: Health,
    movement_bundle: MovementBundle,
    sprite_bundle: SpriteBundle,
    collider: Collider,
    //sprite: Sprite,
    //texture: Handle<Image>,
    marker: Enemy,
}

#[derive(Resource)]
pub struct EnemySpawnConfig {
    pub timer: Timer,
    pub spawn_radius: Vec2,
}

fn enemies_spawn(mut commands: Commands, time: Res<Time>, mut config: ResMut<EnemySpawnConfig>, assets: Res<AppAssets>) {
    config.timer.tick(time.delta());

    // Get a random point on the edge of the circle

    if config.timer.finished() {
        let mut rng = rand::thread_rng();
        let random_angle: f32 = rng.gen_range(0.0..=1000.) * PI * 2.;
        let random_speed: f32 = rng.gen_range(100.0..500.);
        let x = random_angle.cos() * config.spawn_radius.x;
        let y = random_angle.sin() * config.spawn_radius.y;

        commands.spawn(EnemyBundle {
            collider: Collider { radius: 32. },
            movement_bundle: MovementBundle {
                speed: Speed(random_speed),
                ..Default::default()
            },
            sprite_bundle: SpriteBundle {
                texture: assets.enemy1_sprite.clone_weak(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn debug_enemies(q: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (e, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), 16., Color::RED);
    }
}

fn enemies_hate_the_tower(
    mut enemy_q: Query<(&Enemy, &Transform, &mut Velocity)>,
    tower_q: Query<(&Tower, &Transform)>,
) {
    let (_, tower_transform) = tower_q.single();

    for (_, enemy_transform, mut velocity) in enemy_q.iter_mut() {
        let movement_vector = tower_transform.translation.xy() - enemy_transform.translation.xy();
        velocity.0 = movement_vector.normalize_or_zero();
    }
}

fn enemies_damage_the_tower(
    eq: Query<(&Enemy, &Collider, &Transform)>,
    mut tq: Query<(&Tower, &Collider, &Transform, &mut Health)>,
) {
    let (_, tc, tt, mut th) = tq.single_mut();
    for (_, ec, et) in eq.iter() {
        if tc.collides_with(tt, ec, et) {
            th.0 -= 1.;
        }
    }
}

fn flip_enemy_sprite_with_velocity(mut eq: Query<(&Enemy, &Velocity, &mut Sprite)>) {

    for (_, v, mut s) in eq.iter_mut() {
        if v.0.x.is_sign_positive() {
            s.flip_x = true;
        } else {
            s.flip_x = false;
        }
    }
}