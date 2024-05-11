use bevy::prelude::*;
use konnektoren_game::{app_state::AppState, screen, splash::splash_plugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Konnektoren".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, screen::setup)
        // Adds the plugins for each state
        .add_plugins(splash_plugin)
        .run();
}
