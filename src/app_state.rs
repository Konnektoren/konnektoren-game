use bevy::ecs::schedule::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Slides,
    Game,
    Map,
}
