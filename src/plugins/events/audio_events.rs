use bevy::app::Plugin;

use crate::events::{
    pause_audio_event::PauseAudioEvent, play_audio_event::PlayAudioEvent,
    spawn_audio_event::SpawnAudioEvent,
};

pub struct AudioEvents;

impl Plugin for AudioEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnAudioEvent>();
        app.add_event::<PauseAudioEvent>();
        app.add_event::<PlayAudioEvent>();
    }
}
