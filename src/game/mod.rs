use bevy::app::{App, Plugin};

mod options;
mod question;
mod tasks;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tasks::TasksPlugin,
            question::QuestionPlugin,
            options::OptionsPlugin,
        ));
    }
}