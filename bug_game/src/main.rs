use bevy::window::{PresentMode, Cursor};
use bug_lib::{prelude::*, BugGamePlugins};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                title: "Super Bug Smasher ".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest()),
        BugGamePlugins,
    ));

    app.run();
}
