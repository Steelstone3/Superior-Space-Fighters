use bevy::ecs::system::{Res, Resource};

use crate::resources::game_state::GameState;

pub fn run_if_not_paused(game_state: Res<GameState>) -> bool {
    !game_state.paused
}

pub fn run_if_resource_available<T: Resource>(resource: Option<Res<T>>) -> bool {
    let Some(_) = resource else {
        return false;
    };
    return true;
}
