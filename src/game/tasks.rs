use bevy::prelude::*;

use crate::{app_state::AppState, game_state::GameState, prelude::despawn_screen};

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_screen::<TasksEntity>);
    }
}

#[derive(Component)]
struct TasksEntity;

fn setup(mut commands: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_justification = JustifyText::Center;

    let style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(20.0),
        right: Val::Px(20.0),
        ..default()
    };

    let text = format!("Task: {}", game_state.current_task_index + 1);

    commands.spawn((
        TextBundle::from_section(text, text_style)
            .with_text_justify(text_justification)
            .with_style(style),
        TasksEntity,
    ));
}

fn update(mut query: Query<(&TasksEntity, &mut Text)>, game_state: Res<GameState>) {
    for (_, mut text) in &mut query.iter_mut() {
        text.sections[0].value = format!("Task: {}", game_state.current_task_index + 1);
    }
}
