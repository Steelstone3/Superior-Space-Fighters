use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::player::{
    player_sound_effects_plugin::PlayerSoundEffectsPlugin, player_start_plugin::PlayerStartPlugin,
    player_update_plugin::PlayerUpdatePlugin,
};

pub struct PlayerPluginGroup;

impl PluginGroup for PlayerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerUpdatePlugin)
            .add(PlayerStartPlugin)
            .add(PlayerSoundEffectsPlugin)
    }
}
