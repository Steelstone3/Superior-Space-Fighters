use bevy::app::Plugin;

use crate::events::audio_events::{PauseAudioEvent, PlayAudioEvent, SpawnAudioEvent};

pub struct AudioEvents;

impl Plugin for AudioEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnAudioEvent>();
        app.add_event::<PauseAudioEvent>();
        app.add_event::<PlayAudioEvent>();
    }
}
