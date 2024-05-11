use bevy::{asset::AssetMetaCheck, prelude::*};
use konnektoren_game::{
    app_state::AppState, challenge::challenge_plugin, game_state::GameState, map::map_plugin,
    screen, slides::slides_plugin, splash::splash_plugin,
};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // fix for asset loading in wasm
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Konnektoren".to_string(),
                resolution: (800.0, 600.0).into(),
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(GameState::default())
        .init_state::<AppState>()
        .add_systems(Startup, screen::setup)
        .add_plugins((splash_plugin, slides_plugin, map_plugin, challenge_plugin))
        .run();
}
