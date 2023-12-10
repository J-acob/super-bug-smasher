use bevy::prelude::*;

use crate::{collision::Collider, movement::MovementBundle};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileSpawnEvent>().add_systems(
            Update,
            (projectile_emitters_emit, projectiles_spawn).chain(),
        );
    }
}

#[derive(Component, Default, Clone)]
pub struct Projectile;

#[derive(Bundle, Default, Clone)]
pub struct ProjectileBundle {
    pub texture: Handle<Image>,
    pub movement_bundle: MovementBundle,
    pub collider: Collider,
    pub marker: Projectile,
}

#[derive(Component)]
pub struct ProjectileEmitter<'a> {
    pub projectile_bundle: &'a ProjectileBundle,
    pub amount: i32,
    pub timer: Timer,
}

#[derive(Event)]
pub struct ProjectileSpawnEvent {
    projectile_bundle: ProjectileBundle,
}

pub fn projectiles_spawn(mut evt: EventReader<ProjectileSpawnEvent>, mut commands: Commands) {
    for e in evt.read() {
        commands.spawn((e.projectile_bundle.clone(),));
    }
}

pub fn projectile_emitters_emit(
    mut peq: Query<&mut ProjectileEmitter<'static>>,
    mut pewr: EventWriter<ProjectileSpawnEvent>,
    time: Res<Time>,
) {
    for mut pe in peq.iter_mut() {
        pe.timer.tick(time.delta());

        if pe.timer.finished() {
            for _ in 0..(pe.amount) {
                pewr.send(ProjectileSpawnEvent {
                    projectile_bundle: pe.projectile_bundle.clone(),
                });
            }
        }
    }
}

/// After some time, projectiles should despawn
pub fn projectiles_despawn() {}

/// Projectiles do damage to entities
pub fn projectiles_damage_enemies() {}
