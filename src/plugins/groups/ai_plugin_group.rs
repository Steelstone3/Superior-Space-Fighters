use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::ai::{
    ai_save_types_plugin::AISaveTypesPlugin, ai_update_plugin::AIUpdatePlugin,
};

pub struct AIPluginGroup;

impl PluginGroup for AIPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AIUpdatePlugin)
            .add(AISaveTypesPlugin)
    }
}
