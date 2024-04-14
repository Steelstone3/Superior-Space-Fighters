use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{
    ai::ai_plugin::AIPlugin,
    events::{audio_events::AudioEvents, audio_events_handlers::AudioEventHandlers},
    resources::sound::MusicResources,
    start::music::MusicPlugin,
};

pub struct AudioPluginGroup;

impl PluginGroup for AudioPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(AIPlugin)
    }
}
