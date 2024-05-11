use bevy::{
    app::{App, Update},
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        query::{Changed, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    log,
    prelude::default,
    render::color::Color,
    text::{Font, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, AlignSelf, FlexDirection, Interaction, JustifyContent, PositionType, Style,
        UiRect, Val,
    },
};
use konnektoren_core::{
    challenges::{ChallengeResult, ChallengeType},
    commands::{game_commands::SolveOptionCommand, GameCommand},
};

use crate::{app_state::AppState, game_state::GameState, prelude::despawn_screen};

pub fn challenge_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), challenge_setup)
        .add_systems(
            Update,
            challenge_interaction_system.run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, update_results.run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), despawn_screen::<ChallengeScreen>);
}

#[derive(Component)]
struct ChallengeScreen;

#[derive(Component)]
struct OptionNode {
    index: usize,
    name: String,
}

#[derive(Component)]
struct ChallengeResultText(String);

fn create_button(commands: &mut ChildBuilder, index: usize, text: &str, font: Handle<Font>) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::rgb(0.25, 0.25, 0.25).into(),
                ..default()
            },
            OptionNode {
                index,
                name: text.to_string(),
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn challenge_setup(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) {
    let text = format!(
        "Challenge: {}\n\n{}",
        game_state.challenge.challenge_config.name,
        game_state.challenge.challenge_config.description
    );

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            ChallengeScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(20.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::End,
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                })
                .with_children(|buttons| match game_state.challenge.challenge_type {
                    ChallengeType::MultipleChoice(ref dataset) => {
                        for (index, option) in dataset.options.iter().enumerate() {
                            let text = format!("{}. {}", index + 1, option.name);
                            create_button(
                                buttons,
                                index,
                                &text,
                                asset_server.load("fonts/FiraSans-Bold.ttf"),
                            );
                        }
                    }
                });
        });
}

fn challenge_interaction_system(
    mut interaction_query: Query<(&Interaction, &OptionNode), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, option) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let command = SolveOptionCommand {
                option_index: option.index,
            };

            command.execute(&mut game_state).unwrap();
            log::info!("Option selected: {}", option.name);
        }
    }
}

fn update_results(mut commands: Commands, game_state: Res<GameState>) {
    let challenge = &game_state.challenge;

    let results: Vec<String> = match (&challenge.challenge_type, &challenge.challenge_result) {
        (ChallengeType::MultipleChoice(dataset), ChallengeResult::MultipleChoice(options)) => {
            dataset.questions.iter().zip(options.iter()).fold(
                Vec::new(),
                |mut acc, (question, option)| {
                    let correct = question.option == option.id;
                    let result = if correct { "Correct" } else { "Incorrect" };
                    acc.push(format!("{}: {}", question.question, result));
                    acc
                },
            )
        }

        _ => Vec::new(),
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        ChallengeResultText(results.join("\n")),
    ));
}
