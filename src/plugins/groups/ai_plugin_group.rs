use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::ai::ai_plugin::AIPluginGroup;

pub struct AudioPluginGroup;

impl PluginGroup for AudioPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(AIPluginGroup)
    }
}
