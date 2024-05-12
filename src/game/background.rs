use bevy::prelude::*;

use crate::{app_state::AppState, prelude::despawn_screen};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnExit(AppState::Game), despawn_screen::<GameBackground>);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            z_index: ZIndex::Local(-1),
            ..default()
        })
        .insert(GameBackground);
}

#[derive(Component)]
pub struct GameBackground;
