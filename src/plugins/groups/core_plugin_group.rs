use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{
    core::{core_events::CoreEvents, core_resources::CoreResources, player_camera::PlayerCamera},
    events::logging_event_handlers::LoggingEventHandlers,
};

pub struct CorePluginGroup;

impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LoggingEventHandlers)
            .add(CoreEvents)
            .add(CoreResources)
            .add(PlayerCamera)
    }
}
