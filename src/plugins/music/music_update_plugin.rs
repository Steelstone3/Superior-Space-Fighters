use crate::{
    states::core_states::GameState, systems::music::exploration_music::play_exploration_music,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

pub struct MusicUpdatePlugin;

impl Plugin for MusicUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            play_exploration_music.run_if(in_state(GameState::InGame)),
        );
    }
}
