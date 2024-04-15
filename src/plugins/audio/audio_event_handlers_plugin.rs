use bevy::app::{Plugin, Update};

use crate::systems::event_handlers::audio_event_handlers::{pause_audio, play_audio, spawn_audio};

pub struct AudioEventHandlersPlugin;

impl Plugin for AudioEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (play_audio, pause_audio, spawn_audio));
    }
}
