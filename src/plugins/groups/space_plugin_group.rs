use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::start::spawn_world::SpawnWorld;

pub struct SpacePluginGroup;

impl PluginGroup for SpacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(SpawnWorld)
    }
}
