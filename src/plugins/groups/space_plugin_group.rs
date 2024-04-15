use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::space::{maintain_space::MaintainSpacePlugin, spawn_space::SpawnSpacePlugin};

pub struct SpacePluginGroup;

impl PluginGroup for SpacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnSpacePlugin)
            .add(MaintainSpacePlugin)
    }
}
