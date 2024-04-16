use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::collisions::{
    colliison_sound_effects_plugin::CollisionSoundEffectsPlugin,
    collision_event_handlers_plugin::CollisionEventHandlersPlugin,
    collision_events_plugin::CollisionEventsPlugin, collision_plugin::CollisionPlugin,
};

pub struct CollisionPluginGroup;

impl PluginGroup for CollisionPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CollisionPlugin)
            .add(CollisionEventsPlugin)
            .add(CollisionEventHandlersPlugin)
            .add(CollisionSoundEffectsPlugin)
    }
}
