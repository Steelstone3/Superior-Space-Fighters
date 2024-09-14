use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::music::music_plugin::MusicPlugin;

pub struct MusicPluginGroup;

impl PluginGroup for MusicPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(MusicPlugin)
    }
}
