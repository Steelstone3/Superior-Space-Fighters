use crate::{
    states::core_states::GameState, systems::music::exploration_music::play_exploration_music,
};
use bevy::{app::Plugin, prelude::OnExit};

pub struct MusicUpdatePlugin;

impl Plugin for MusicUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnExit(GameState::MainMenu), play_exploration_music);
    }
}
