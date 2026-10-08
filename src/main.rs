use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup},
    utils::default,
    window::{Window, WindowPlugin},
};

mod camera;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),
        camera::Plugin,
    ));

    app.run();
}
