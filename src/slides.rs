use crate::{app_state::AppState, prelude::despawn_screen};
use bevy::prelude::*;

const SLIDES: [&str; 6] = [
    "slides/1.png",
    "slides/3.png",
    "slides/6.png",
    "slides/7.png",
    "slides/8.png",
    "slides/9.png",
];

#[derive(Component)]
struct SlideshowScreen;

#[derive(Resource, Deref, DerefMut)]
struct SlideImage(usize);

pub fn slides_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Slides), setup_slideshow)
        .add_systems(Update, navigate_slides.run_if(in_state(AppState::Slides)))
        .add_systems(OnExit(AppState::Slides), despawn_screen::<SlideshowScreen>);
}

fn create_button(commands: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::rgb(0.25, 0.25, 0.25).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn setup_slideshow(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_slide = asset_server.load("slides/1.png");

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
            SlideshowScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(400.0),
                        margin: UiRect::all(Val::Auto),
                        flex_shrink: 0.0,
                        ..default()
                    },
                    image: UiImage::new(first_slide),
                    ..default()
                },
                SlideshowScreen,
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
                .with_children(|buttons| {
                    create_button(
                        buttons,
                        "Previous",
                        asset_server.load("fonts/FiraSans-Bold.ttf"),
                    );
                    create_button(
                        buttons,
                        "Next",
                        asset_server.load("fonts/FiraSans-Bold.ttf"),
                    );
                    create_button(
                        buttons,
                        "Skip",
                        asset_server.load("fonts/FiraSans-Bold.ttf"),
                    );
                });
        });

    commands.insert_resource(SlideImage(0));
}

fn navigate_slides(
    mut slide_image_res: ResMut<SlideImage>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&Text>,
    mut state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut image_query: Query<&mut UiImage, With<SlideshowScreen>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get_mut(children[0]) {
                match text.sections[0].value.as_str() {
                    "Previous" => {
                        if slide_image_res.0 > 0 {
                            slide_image_res.0 -= 1;
                        } else {
                            slide_image_res.0 = SLIDES.len() - 1;
                        }
                    }
                    "Next" => {
                        slide_image_res.0 = (slide_image_res.0 + 1) % SLIDES.len();
                    }
                    "Skip" => {
                        state.set(AppState::Map);
                        return;
                    }
                    _ => {}
                }

                if let Ok(mut ui_image) = image_query.get_single_mut() {
                    *ui_image = UiImage::new(asset_server.load(SLIDES[slide_image_res.0]));
                }
            }
        }
    }
}
