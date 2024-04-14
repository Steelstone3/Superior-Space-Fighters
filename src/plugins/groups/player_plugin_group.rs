use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::player::player_plugin::PlayerPlugin;

pub struct PlayerPluginGroup;

impl PluginGroup for PlayerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerPlugin)
    }
}
