use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{
    audio::audio_plugin::AudioPlugin,
    camera::camera_plugin::CameraPlugin,
    core::{
        core_events::CoreEvents, core_events_handlers::CoreEventHandlers,
        core_resources::CoreResources,
    },
    sprite::sprite_plugin::SpritePlugin,
};

pub struct CorePluginGroup;

impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreEventHandlers)
            .add(CoreEvents)
            .add(CoreResources)
            .add(CameraPlugin)
            .add(SpritePlugin)
            .add(AudioPlugin)
    }
}
