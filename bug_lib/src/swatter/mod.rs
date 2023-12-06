use bevy::{
    prelude::*,
    ui::{widget::UiImageSize, ContentSize, FocusPolicy},
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    collision::{visualize_colliders, Collider},
    enemy::Enemy, movement::velocity_moves_transforms,
};

pub struct SwatterPlugin;

impl Plugin for SwatterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (swatter_follows_mouse.before(visualize_colliders), swatter_despawns_enemies.after(swatter_follows_mouse).after(velocity_moves_transforms)))
            .add_systems(Startup, setup_swatter);
    }
}

#[derive(Component)]
pub struct Swatter;

/// Sets up swatter for use
fn setup_swatter(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Make the cursor invisible
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        Swatter,
        Collider { radius: 64. },
        SpatialBundle {
            ..Default::default()
        },
        Node::default(),
        BackgroundColor(Color::BLUE),
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
        UiImage::default(),
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

fn swatter_despawns_enemies(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy, &Collider, &Transform)>,
    swatter_query: Query<(&Swatter, &Collider, &Transform)>,
    buttons: Res<Input<MouseButton>>,
) {
    let Ok((_, swatter_collider, swatter_transform)) = swatter_query.get_single() else {
        return;
    };

    if buttons.pressed(MouseButton::Left) {
        for (e, _, enemy_collider, enemy_transform) in enemy_query.iter() {
            let collision = swatter_collider.collides_with(swatter_transform, enemy_collider, enemy_transform);

            if collision {
                commands.entity(e).despawn_recursive()
            } else {
                //println!("No collision");
            }
        }
    }
}
