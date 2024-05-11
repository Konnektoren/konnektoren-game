use bevy::{
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GameState(konnektoren_core::game::GameState);
