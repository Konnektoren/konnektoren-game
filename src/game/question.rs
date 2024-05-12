use bevy::prelude::*;
use konnektoren_core::challenges::ChallengeType;

use crate::{app_state::AppState, game_state::GameState, prelude::despawn_screen};

pub struct QuestionPlugin;

impl Plugin for QuestionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_screen::<QuestionEntity>);
    }
}

#[derive(Component)]
struct QuestionEntity;

fn setup(mut commands: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_justification = JustifyText::Center;

    let style = Style {
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        position_type: PositionType::Relative,
        top: Val::Percent(20.0),
        left: Val::Percent(20.0),
        ..default()
    };

    let current_question = game_state.current_task_index;

    let text = match game_state.challenge.challenge_type {
        ChallengeType::MultipleChoice(ref dataset) => {
            let question = dataset.questions.get(current_question).unwrap();

            format!("Question: {}\n\n{}", question.question, question.help)
        }
    };

    commands.spawn((
        TextBundle::from_section(text, text_style)
            .with_text_justify(text_justification)
            .with_style(style),
        QuestionEntity,
    ));
}

fn update(mut query: Query<(&QuestionEntity, &mut Text)>, game_state: Res<GameState>) {
    for (_, mut text) in &mut query.iter_mut() {
        let current_question = game_state.current_task_index;

        let question = match game_state.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let question = dataset.questions.get(current_question).unwrap();

                format!("Question: {}\n\n{}", question.question, question.help)
            }
        };

        text.sections[0].value = question;
    }
}
