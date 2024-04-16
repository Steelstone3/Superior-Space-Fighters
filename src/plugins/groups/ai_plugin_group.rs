use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::ai::ai_plugin::AIPlugin;

pub struct AIPluginGroup;

impl PluginGroup for AIPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(AIPlugin)
    }
}
