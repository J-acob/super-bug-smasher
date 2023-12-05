use bevy::{prelude::*, window::{PrimaryWindow, CursorGrabMode}, ui::widget::UiImageSize};

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
        ImageBundle {
            /* 
            text: Text::from_section("Hi", TextStyle {
                font_size: 40.,
                ..Default::default()
            }),
            */
            background_color: Color::BLUE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(32.),
                height: Val::Px(32.),
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        }
    ))
    ;
}

/// Moves the swatter to the location of the mouse
fn swatter_follows_mouse(
    mut swatter_query: Query<(&Swatter, &mut Style)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // There should only ever be ONE swatter (unless multiplayer..?)
    let Ok((_, mut swatter_style)) = swatter_query.get_single_mut() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let mut window = windows.single_mut();

    if let Some(mouse_position) = window
        .cursor_position()
        //.and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        window.cursor.visible = false;
        swatter_style.top = Val::Px(mouse_position.y);
        swatter_style.left = Val::Px(mouse_position.x);
         
    } else {
        // If we can't get cursor position, make it visible again
        window.cursor.visible = true;
        // Also reset the position of stuff so the user can't do any _weird_ stuff
    }
}
