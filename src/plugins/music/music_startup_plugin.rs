use crate::systems::music::exploration_music::play_exploration_music;
use bevy::app::{Plugin, Startup};

pub struct MusicStartupPlugin;

impl Plugin for MusicStartupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, play_exploration_music);
    }
}
