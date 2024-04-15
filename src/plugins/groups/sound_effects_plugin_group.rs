use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::sound_effects::sound_effects_plugin::SoundEffectsPlugin;

pub struct SoundEffectsPluginGroup;

impl PluginGroup for SoundEffectsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(SoundEffectsPlugin)
    }
}
