use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::player::{
    player_save_types_plugins::PlayerSaveTypesPlugin,
    player_sound_effects_update_plugin::PlayerSoundEffectsUpdatePlugin,
    player_update_plugin::PlayerUpdatePlugin,
};

pub struct PlayerPluginGroup;

impl PluginGroup for PlayerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerUpdatePlugin)
            .add(PlayerSoundEffectsUpdatePlugin)
            .add(PlayerSaveTypesPlugin)
    }
}
