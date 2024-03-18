use bevy::prelude::{App, Plugin};

use super::{
    camera_resources::CameraResources, combat_resources::CombatResources,
    sound_resources::SoundResources, trading_resources::TradingResources,
    world_resources::WorldResources,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldResources)
            .add_plugins(TradingResources)
            .add_plugins(CombatResources)
            .add_plugins(CameraResources)
            .add_plugins(SoundResources);
    }
}
