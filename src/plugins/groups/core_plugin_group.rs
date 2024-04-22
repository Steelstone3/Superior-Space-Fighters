use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{
    audio::audio_plugin::AudioPlugin,
    camera::camera_plugin::CameraPlugin,
    core::{
        core_events_handlers_plugin::CoreEventHandlersPlugin, core_events_plugin::CoreEventsPlugin,
        core_resources_plugin::CoreResourcesPlugin, core_save_types_plugin::CoreSaveTypesPlugin,
        core_states_plugin::CoreStatesPlugin,
    },
    sprite::sprite_plugin::SpritePlugin,
};

pub struct CorePluginGroup;

impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreEventHandlersPlugin)
            .add(CoreEventsPlugin)
            .add(CoreResourcesPlugin)
            .add(CameraPlugin)
            .add(SpritePlugin)
            .add(AudioPlugin)
            .add(CoreStatesPlugin)
            .add(CoreSaveTypesPlugin)
    }
}
