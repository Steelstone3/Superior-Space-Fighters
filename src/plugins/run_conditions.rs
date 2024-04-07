use bevy::ecs::system::Res;

use crate::resources::game_state::GameState;

pub fn run_if_not_paused(game_state: Res<GameState>) -> bool {
    return !game_state.paused;
}
