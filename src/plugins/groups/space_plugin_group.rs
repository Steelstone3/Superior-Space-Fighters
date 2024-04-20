use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::space::{
    maintain_space_plugin::MaintainSpacePlugin, space_save_types::SpaceSaveTypesPlugin,
    spawn_space_plugin::SpawnSpacePlugin,
};

pub struct SpacePluginGroup;

impl PluginGroup for SpacePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnSpacePlugin)
            .add(MaintainSpacePlugin)
            .add(SpaceSaveTypesPlugin)
    }
}
