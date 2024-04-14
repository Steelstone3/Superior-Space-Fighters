use bevy::app::{Plugin, Startup};

use crate::systems::music::exploration_music::play_exploration_music;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, play_exploration_music);
    }
}
