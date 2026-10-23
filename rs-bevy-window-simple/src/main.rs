use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup},
    window::{Window, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Window".into(),
                resolution: (800, 800).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
