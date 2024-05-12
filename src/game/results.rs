use bevy::prelude::*;
use konnektoren_core::challenges::{ChallengeResult, ChallengeType};

use crate::{app_state::AppState, game_state::GameState, prelude::despawn_screen};

pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayedResults>()
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_screen::<ResultsEntity>);
    }
}

#[derive(Component)]
struct ResultsEntity;

#[derive(Default, Resource)]
struct DisplayedResults {
    count: usize,
}

fn calculate_results(game_state: &GameState) -> Vec<String> {
    match (
        &game_state.challenge.challenge_type,
        &game_state.challenge.challenge_result,
    ) {
        (ChallengeType::MultipleChoice(dataset), ChallengeResult::MultipleChoice(options)) => {
            dataset.questions.iter().zip(options.iter()).fold(
                Vec::new(),
                |mut acc, (question, option)| {
                    let correct = if question.option == option.id {
                        format!("Correct: {}", question.question)
                    } else {
                        format!("Incorrect: {}", question.question)
                    };
                    acc.push(correct);
                    acc
                },
            )
        }
    }
}

fn calculate_score(results: &[String], game_state: &GameState) -> f32 {
    let num_tasks = match &game_state.challenge.challenge_type {
        ChallengeType::MultipleChoice(dataset) => dataset.questions.len(),
    };

    let score = results
        .iter()
        .filter(|line| line.contains("Correct"))
        .count() as u32;

    score as f32 / num_tasks as f32 * 100.0
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut displayed_results: ResMut<DisplayedResults>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    commands
        .spawn(TextBundle {
            text: Text::from_section("Calculating score...", text_style.clone())
                .with_justify(JustifyText::Center),
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        })
        .insert(ResultsEntity);

    displayed_results.count = 0;
}

fn update(
    mut commands: Commands,
    mut results_query: Query<(Entity, &mut Text), With<ResultsEntity>>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut displayed_results: ResMut<DisplayedResults>,
) {
    let results = calculate_results(&game_state);
    let score = calculate_score(&results, &game_state);

    let mut entity_iter = results_query.iter_mut();
    if let Some((_, mut overall_text)) = entity_iter.next() {
        overall_text.sections[0].value = format!(
            "You have completed the challenge with a score of {:.1}!",
            score
        );
    }

    let start_y = 60.0;
    let line_height = 30.0;

    for (index, result) in results.iter().enumerate() {
        if index >= displayed_results.count {
            commands
                .spawn(TextBundle {
                    text: Text::from_section(
                        result.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    style: Style {
                        align_self: AlignSelf::Center,
                        position_type: PositionType::Absolute,
                        top: Val::Px(start_y + index as f32 * line_height),
                        ..default()
                    },
                    ..default()
                })
                .insert(ResultsEntity);
        } else {
            if let Some((entity, mut text)) = entity_iter.next() {
                text.sections[0].value = result.clone();
                commands.entity(entity).insert(Style {
                    top: Val::Px(start_y + index as f32 * line_height),
                    ..default()
                });
            }
        }
    }

    displayed_results.count = results.len();
}
