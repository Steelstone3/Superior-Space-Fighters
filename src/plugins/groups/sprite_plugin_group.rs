use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::events::{
    sprite_event_handlers::SpriteEventHandlers, sprite_events::SpriteEvents,
};

pub struct SpritePluginGroup;

impl PluginGroup for SpritePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpriteEvents)
            .add(SpriteEventHandlers)
    }
}
