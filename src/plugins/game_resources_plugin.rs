use crate::resources::fleet_credits::FleetCredits;
use bevy::prelude::{App, Plugin};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FleetCredits(1000.0));
    }
}
