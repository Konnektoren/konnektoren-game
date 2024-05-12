pub mod app_state;
pub mod game;
pub mod game_state;
pub mod map;
pub mod screen;
pub mod slides;
pub mod splash;

pub mod prelude {
    pub use crate::screen::despawn_screen;
}
