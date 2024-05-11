use bevy::{log, prelude::*};
use konnektoren_core::challenges::ChallengeConfig;

use crate::{
    app_state::{self, AppState},
    game_state::GameState,
    prelude::despawn_screen,
};

#[derive(Component)]
struct OnMapScreen;

pub fn map_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Map), map_setup)
        .add_systems(
            Update,
            challenge_interaction_system.run_if(in_state(AppState::Map)),
        )
        .add_systems(OnExit(AppState::Map), despawn_screen::<MapEntity>);
}

#[derive(Component)]
struct MapEntity;

#[derive(Component)]
struct MapLine;

#[derive(Component)]
struct ChallengeNode((usize, String));

fn calculate_bounds(challenges: &[(String, i32, i32)]) -> ([i32; 2], [i32; 2]) {
    let x_min = challenges
        .iter()
        .map(|(_, x, _)| *x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let x_max = challenges
        .iter()
        .map(|(_, x, _)| *x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let y_min = challenges
        .iter()
        .map(|(_, _, y)| *y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let y_max = challenges
        .iter()
        .map(|(_, _, y)| *y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);

    ([x_min - 1, x_max + 2 * 1], [2 * y_min - 1, y_max + 4 * 1])
}

fn map_setup(mut commands: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
    let window_width = 800.0; //window.width();
    let window_height = 600.0; //window.height();

    let challenges = game_state
        .game
        .game_path
        .challenges
        .iter()
        .map(|challenge| {
            (
                challenge.name.clone(),
                challenge.position.unwrap_or_default().0,
                challenge.position.unwrap_or_default().1,
            )
        })
        .collect::<Vec<_>>();

    let (x_bounds, y_bounds) = calculate_bounds(&challenges);
    let scale_x = window_width / (x_bounds[1] - x_bounds[0]) as f32;
    let scale_y = window_height / (y_bounds[1] - y_bounds[0]) as f32;
    let scale = scale_x.min(scale_y);

    let parent_entity = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .id();

    for (challenge_index, challenge) in game_state.game.game_path.challenges.iter().enumerate() {
        commands.entity(parent_entity).with_children(|parent| {
            add_challenge(
                parent,
                challenge,
                challenge_index,
                asset_server.load("fonts/FiraSans-Bold.ttf"),
                scale,
                x_bounds,
                y_bounds,
            );
        });
    }
}

fn add_challenge(
    commands: &mut ChildBuilder,
    challenge: &ChallengeConfig,
    challenge_index: usize,
    font: Handle<Font>,
    scale: f32,
    x_bounds: [i32; 2],
    y_bounds: [i32; 2],
) {
    let pos_x = ((challenge.position.unwrap_or_default().0 - x_bounds[0]) as f32) * scale;
    let pos_y = ((challenge.position.unwrap_or_default().1 - y_bounds[0]) as f32) * scale;

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(pos_x),
                    top: Val::Px(pos_y),
                    width: Val::Px(1.0 * scale),
                    height: Val::Px(1.0 * scale),
                    ..default()
                },
                background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                ..default()
            },
            ChallengeNode((challenge_index, challenge.id.clone())),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &challenge.name,
                TextStyle {
                    font,
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            ));
        })
        .insert(MapEntity);
}

fn challenge_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &ChallengeNode),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, challenge_node) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match game_state.game.create_challenge(&challenge_node.0 .1) {
                Ok(challenge) => {
                    game_state.challenge = challenge;
                    app_state.set(AppState::Game);
                    log::info!("Pressed challenge {}", challenge_node.0 .1);
                }
                Err(e) => {
                    log::error!("Error creating challenge: {}", e);
                }
            },
            Interaction::Hovered => {
                // Optionally handle hover state
            }
            Interaction::None => {
                // Optionally handle the transition to no interaction
            }
            _ => {}
        }
    }
}
