use bevy::app::Plugin;

use crate::resources::fleet_credits::FleetCredits;

pub struct TradingResourcesPlugin;

impl Plugin for TradingResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(FleetCredits(1000));
    }
}
