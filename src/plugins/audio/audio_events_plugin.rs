use bevy::app::Plugin;

use crate::events::audio_events::{PauseAudioEvent, PlayAudioEvent, SpawnAudioEvent};

pub struct AudioEventsPlugin;

impl Plugin for AudioEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnAudioEvent>()
            .add_event::<PauseAudioEvent>()
            .add_event::<PlayAudioEvent>();
    }
}
