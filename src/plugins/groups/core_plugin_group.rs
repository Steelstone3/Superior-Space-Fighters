use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{
    audio::audio_plugin::AudioPlugin,
    core::{core_events::CoreEvents, core_resources::CoreResources, player_camera::PlayerCamera},
    events::logging_event_handlers::LoggingEventHandlers,
    sprite::sprite_plugin::SpritePlugin,
};

pub struct CorePluginGroup;

impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LoggingEventHandlers)
            .add(CoreEvents)
            .add(CoreResources)
            .add(PlayerCamera)
            .add(SpritePlugin)
            .add(AudioPlugin)
    }
}
