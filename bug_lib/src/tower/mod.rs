use bevy::prelude::*;

use crate::{
    asset_loading::AppAssets, collision::Collider, combat::prelude::Health,
    enemy::enemies_damage_the_tower, state::AppState, ui::despawn_screen,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            (setup_tower, apply_deferred, setup_tower_health_ui),
        )
        .add_systems(
            OnTransition {
                from: AppState::InGame,
                to: AppState::GameOver,
            },
            despawn_screen::<TowerHealthBarUi>,
        )
        .add_systems(
            Update,
            (
                debug_tower,
                tower_health_bar_updates.after(enemies_damage_the_tower),
            )
                .distributive_run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Default)]
pub struct Tower;

#[derive(Bundle, Default)]
pub struct TowerBundle {
    marker: Tower,
    health: Health,
    collider: Collider,
    transform: Transform,
}

/// Spawns the tower
fn setup_tower(mut commands: Commands) {
    println!("Setting up tower!");
    commands.spawn(TowerBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        collider: Collider { radius: 16. },
        health: Health(1000.),
        ..default()
    });
}

/// Shows a thingy to represent the tower
fn debug_tower(q: Query<(&Tower, &Transform)>, mut gizmos: Gizmos) {
    for (_, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), 16., Color::GREEN);
    }
}

#[derive(Component)]
pub struct TowerHealthBarUi;

#[derive(Component)]
pub struct TowerHealthBarUiValue;

/// Shows the tower's health
fn setup_tower_health_ui(mut commands: Commands, assets: Res<AppAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(90.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,
                    ..Default::default()
                },
                ..Default::default()
            },
            TowerHealthBarUi,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Base Health",
                TextStyle {
                    font: assets.font.clone_weak(),
                    color: Color::WHITE,
                    font_size: 40.,
                    ..Default::default()
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(90.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,
                    ..Default::default()
                },
                ..Default::default()
            },
            TowerHealthBarUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        max_width: Val::Px(1000.),
                        ..Default::default()
                    },
                    z_index: ZIndex::Global(-1),
                    background_color: BackgroundColor(Color::GREEN),
                    ..Default::default()
                },
                TowerHealthBarUiValue,
            ));
        });
}

fn tower_health_bar_updates(
    thq: Query<(&Tower, &Health)>,
    mut thbq: Query<(&TowerHealthBarUiValue, &mut Style)>,
) {
    let (_, th) = thq.single();
    let (_, mut s) = thbq.single_mut();

    s.width = Val::Percent((th.0 / 1000.) * 10.);
}
