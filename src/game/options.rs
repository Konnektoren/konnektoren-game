use bevy::{log, prelude::*};
use konnektoren_core::{
    challenges::ChallengeType,
    commands::{game_commands::SolveOptionCommand, GameCommand},
};

use crate::{app_state::AppState, game_state::GameState, prelude::despawn_screen};

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_screen::<OptionsEntity>);
    }
}

#[derive(Component)]
struct OptionsEntity;

#[derive(Component)]
struct OptionNode {
    index: usize,
    name: String,
}

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

fn setup(mut commands: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
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
            OptionsEntity,
        ))
        .with_children(|parent| {
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

fn update(
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
