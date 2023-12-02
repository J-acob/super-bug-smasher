use template_lib::{prelude::*, TemplatePlugins};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                title: "Template".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        TemplatePlugins,
    ));

    app.run();
}
