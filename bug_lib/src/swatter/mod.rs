use std::time::Duration;

use bevy::{
    prelude::*,
    ui::{widget::UiImageSize, ContentSize, FocusPolicy},
    window::{CursorGrabMode, PrimaryWindow}, audio::PlaybackMode,
};

use crate::{
    asset_loading::AppAssets,
    collision::{visualize_colliders, Collider},
    combat::prelude::{Flasher, Health},
    enemy::Enemy,
    movement::velocity_moves_transforms,
    state::AppState,
};

pub struct SwatterPlugin;

impl Plugin for SwatterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                swatter_follows_mouse.before(visualize_colliders),
                swatter_damages_enemy
                    .after(swatter_follows_mouse)
                    .after(velocity_moves_transforms).run_if(in_state(AppState::InGame)),
                enemies_die,
                apply_deferred,
            )
                .chain(),
        )
        .add_systems(
            OnTransition {
                from: AppState::AssetsLoading,
                to: AppState::MainMenu,
            },
            setup_swatter.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            flip_swatter_ui_image
                .run_if(in_state(AppState::InGame))
                .after(swatter_follows_mouse),
        );
    }
}

#[derive(Component)]
pub struct Swatter;

/// Sets up swatter for use
fn setup_swatter(mut commands: Commands, mut windows: Query<&mut Window>, assets: Res<AppAssets>) {
    let robot1_sprite = &assets.robot1_sprite;
    // Make the cursor invisible
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        Swatter,
        Collider { radius: 16. },
        SpatialBundle {
            ..Default::default()
        },
        Node::default(),
        BackgroundColor(Color::WHITE),
        Style {
            position_type: PositionType::Absolute,
            width: Val::Px(32.),
            height: Val::Px(32.),
            margin: UiRect {
                top: Val::Auto,
                left: Val::Auto,
                ..Default::default()
            },
            ..Default::default()
        },
        ZIndex::Global(1),
        FocusPolicy::default(),
        UiImage {
            texture: robot1_sprite.clone_weak(),
            ..Default::default()
        },
        ContentSize::default(),
        UiImageSize::default(),
    ));
}

pub fn swatter_follows_mouse(
    mut swatter_query: Query<(&Swatter, &mut Transform, &mut Style)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // There should only ever be ONE swatter (unless multiplayer..?)
    let Ok((_, mut swatter_transform, mut swatter_style)) = swatter_query.get_single_mut() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let mut window = windows.single_mut();

    // Move the transform
    if let Some(mouse_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        swatter_transform.translation = mouse_position.xy().extend(1.);
    }

    // Update cursor graphic position
    if let Some(mouse_position) = window.cursor_position() {
        window.cursor.visible = false;
        // (This is the radius of the collider)
        swatter_style.top = Val::Px(mouse_position.y - 16.);
        swatter_style.left = Val::Px(mouse_position.x - 16.);
    } else {
        // If we can't get cursor position, make it visible again
        window.cursor.visible = true;
    }
}

fn swatter_damages_enemy(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy, &Collider, &Transform, &mut Health)>,
    swatter_query: Query<(&Swatter, &Collider, &Transform)>,
    buttons: Res<Input<MouseButton>>,
    assets: Res<AppAssets>
) {
    let Ok((_, swatter_collider, swatter_transform)) = swatter_query.get_single() else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        for (e, _, enemy_collider, enemy_transform, mut health) in enemy_query.iter_mut() {
            let collision =
                swatter_collider.collides_with(swatter_transform, enemy_collider, enemy_transform);

            if collision {
                commands.entity(e).insert(Flasher(Timer::new(
                    Duration::from_millis(100),
                    TimerMode::Once,
                )));
                health.0 -= 100.;
                commands.spawn(AudioBundle {
                    source: assets.hit_audio.clone_weak(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}

// Swatter should flip depending on if mouse is going left or right
fn flip_swatter_ui_image(
    mut last_swatter_pos: Local<Vec2>,
    mut sq: Query<(&Swatter, &Transform, &mut UiImage)>,
) {
    let (_, t, mut s) = sq.single_mut();
    let direction = (*last_swatter_pos - t.translation.xy()).normalize_or_zero();

    if direction.x != 0. {
        if direction.x.is_sign_negative() {
            s.flip_x = true;
        } else {
            s.flip_x = false;
        }
    }

    *last_swatter_pos = t.translation.xy();
}

fn enemies_die(eq: Query<(Entity, &Enemy, &Health)>, mut commands: Commands) {
    for (e, _, h) in eq.iter() {
        if h.0 <= 0. {
            commands.entity(e).despawn_recursive();
        }
    }
}
