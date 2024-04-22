use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    events::audio_events::{PauseAudioEvent, PlayAudioEvent},
    plugins::run_conditions::event_called,
    systems::event_handlers::audio_event_handlers::{pause_audio, play_audio, spawn_audio},
};

pub struct AudioEventHandlersPlugin;

impl Plugin for AudioEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_audio)
            .add_systems(Update, play_audio.run_if(event_called::<PlayAudioEvent>))
            .add_systems(Update, pause_audio.run_if(event_called::<PauseAudioEvent>));
    }
}
