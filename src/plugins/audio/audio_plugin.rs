use bevy::app::Plugin;

use crate::plugins::{
    events::{audio_events::AudioEvents, audio_events_handlers::AudioEventHandlers},
    resources::sound::MusicResources,
    start::music::MusicPlugin,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((AudioEvents, AudioEventHandlers, MusicResources, MusicPlugin));
    }
}
