use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::trading::trading_resources_plugin::TradingResourcesPlugin;

pub struct TradingPluginGroup;

impl PluginGroup for TradingPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TradingResourcesPlugin)
    }
}
