use bevy::app::Plugin;

use crate::states::core_states::GameState;

pub struct CoreStatesPlugin;

impl Plugin for CoreStatesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(GameState::MainMenu);
    }
}
