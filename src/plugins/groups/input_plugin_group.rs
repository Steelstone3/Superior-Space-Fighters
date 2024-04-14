use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::player::player_movement::PlayerShipMovementPlugin;

pub struct InputPluginGroup;

impl PluginGroup for InputPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerShipMovementPlugin)
    }
}
