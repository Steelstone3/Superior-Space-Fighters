use bevy::app::Plugin;

use crate::resources::game_state::GameState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameState { paused: false });
    }
}
