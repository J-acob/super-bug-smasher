use bevy::{prelude::*, window::PrimaryWindow};

pub struct SwatterPlugin;

impl Plugin for SwatterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, swatter_follows_mouse)
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
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..Default::default()
        },
    ));
}

/// Moves the swatter to the location of the mouse
fn swatter_follows_mouse(
    mut swatter_query: Query<(&Swatter, &mut Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {

    // There should only ever be ONE swatter (unless multiplayer..?)
    let Ok((_, mut swatter_transform)) = swatter_query.get_single_mut() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let window = windows.single();

    if let Some(mouse_world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        swatter_transform.translation = Vec3::new(mouse_world_position.x, mouse_world_position.y, 0.);
    }
}
