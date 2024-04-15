use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::collisions::{
    collision_event_handlers::CollisionEventHandlers, collision_events::CollisionEvents,
    collision_plugin::CollisionPlugin,
};

pub struct CollisionPluginGroup;

impl PluginGroup for CollisionPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CollisionPlugin)
            .add(CollisionEvents)
            .add(CollisionEventHandlers)
    }
}