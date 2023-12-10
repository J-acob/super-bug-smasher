use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::{prelude::*, render::render_resource::Texture, utils::HashMap};

use crate::{
    asset_loading::AppAssets,
    collision::Collider,
    combat::{prelude::Health, read_damage_events},
    game::DifficultyConfig,
    movement::{self, velocity_moves_transforms, MovementBundle, Speed, Velocity},
    state::AppState,
    tower::Tower,
    xp::ExperienceBundle,
};
use rand::{distributions::uniform::SampleRange, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnConfig {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
            spawn_radius: Vec2::new(1920. / 1.5, 1080. / 1.5),
        })
        .add_systems(
            Update,
            ((enemies_spawn,)
                .chain()
                .distributive_run_if(in_state(AppState::InGame))),
        )
        .add_systems(
            Update,
            enemies_hate_the_tower
                .before(velocity_moves_transforms)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (enemies_damage_the_tower, flip_enemy_sprite_with_velocity)
                .distributive_run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            enemies_die
                .after(read_damage_events)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    health: Health,
    movement_bundle: MovementBundle,
    sprite_bundle: SpriteBundle,
    collider: Collider,
    movement_cooldown: MovementCooldown,
    marker: Enemy,
}

#[derive(Default, Clone)]
pub struct EnemyInitData {
    pub sprite: Handle<Image>,
    pub health_range: Range<f32>,
    pub speed_range: Range<f32>,
    //TODO
    pub damage_range: Range<f32>,
    pub movement_cooldown_range: Range<f32>,
    // The required difficulty for this enemy to spawn
    pub required_difficulty: i32,
}

#[derive(Resource)]
pub struct EnemyList(pub HashMap<String, EnemyInitData>);

#[derive(Resource)]
pub struct EnemyPool(pub Vec<EnemyInitData>);

#[derive(Resource)]
pub struct EnemySpawnConfig {
    pub timer: Timer,
    pub spawn_radius: Vec2,
}

#[derive(Component, Default)]
pub struct MovementCooldown(pub Timer);

fn enemies_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EnemySpawnConfig>,
    difficulty_config: Res<DifficultyConfig>,
    enemy_data_pool: Res<EnemyPool>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = rand::thread_rng();

        for eid in enemy_data_pool.0.iter() {
            // Only spawn enemies if they can spawn at the current difficulty
            if eid.required_difficulty <= difficulty_config.difficulty_level {
                for _ in 0..(difficulty_config.enemies_per_spawn_batch as i32
                    * difficulty_config.modifier as i32)
                {
                    // Get random point on edge of spawn circle
                    let random_angle: f32 = rng.gen_range(0.0..=1000.) * PI * 2.;
                    let x = random_angle.cos() * config.spawn_radius.x;
                    let y = random_angle.sin() * config.spawn_radius.y;

                    // Get random monster(s) stats
                    //let random_speed: f32 = rng.gen_range(50.0..100.0) * difficulty_config.modifier;
                    let random_speed: f32 =
                        rng.gen_range(eid.speed_range.clone()) * difficulty_config.modifier;
                    let random_health: f32 =
                        rng.gen_range(eid.health_range.clone()) * difficulty_config.modifier;

                    commands.spawn(EnemyBundle {
                        collider: Collider { radius: 32. },
                        movement_bundle: MovementBundle {
                            speed: Speed(random_speed),
                            ..Default::default()
                        },
                        sprite_bundle: SpriteBundle {
                            texture: eid.sprite.clone_weak(),
                            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                            ..Default::default()
                        },
                        movement_cooldown: MovementCooldown(Timer::new(
                            Duration::from_secs(1),
                            TimerMode::Repeating,
                        )),
                        health: Health(random_health),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn debug_enemies(q: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (e, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), 16., Color::RED);
    }
}

fn enemies_hate_the_tower(
    mut enemy_q: Query<(&Enemy, &Transform, &mut Velocity, &mut MovementCooldown)>,
    tower_q: Query<(&Tower, &Transform)>,
    time: Res<Time>,
) {
    let (_, tower_transform) = tower_q.single();

    for (_, enemy_transform, mut velocity, mut mc) in enemy_q.iter_mut() {
        mc.0.tick(time.delta());

        if mc.0.finished() {
            let movement_vector =
                tower_transform.translation.xy() - enemy_transform.translation.xy();
            velocity.0 = movement_vector.normalize_or_zero();
        }
    }
}

pub fn enemies_damage_the_tower(
    eq: Query<(&Enemy, &Collider, &Transform)>,
    mut tq: Query<(&Tower, &Collider, &Transform, &mut Health)>,
) {
    let (_, tc, tt, mut th) = tq.single_mut();
    for (_, ec, et) in eq.iter() {
        if tc.collides_with(tt, ec, et) {
            th.0 -= 0.1;
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

fn enemies_die(
    eq: Query<(Entity, &Enemy, &Health, &Transform)>,
    mut commands: Commands,
    assets: Res<AppAssets>,
) {
    let mut rng = thread_rng();
    for (e, _, h, et) in eq.iter() {
        if h.0 <= 0. {
            // Despawn the entity
            commands.entity(e).despawn_recursive();

            // Drop some experience
            commands.spawn(ExperienceBundle {
                collider: Collider { radius: 16. },
                sprite_bundle: SpriteBundle {
                    texture: assets.bug_core.clone_weak(),
                    transform: Transform::from_xyz(
                        et.translation.x + rng.gen_range(-100.0..100.00),
                        et.translation.y + rng.gen_range(-100.0..100.00),
                        1.,
                    ),
                    ..Default::default()
                },
                ..default()
            });
        }
    }
}
