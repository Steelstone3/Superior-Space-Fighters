use crate::resources::{
    blaster_ammunition::BlasterAmmunition, camera_settings::CameraSettings, chunks::Chunks,
    combat_guild_reputation::CombatGuildReputation, exotic_ammunition::ExoticAmmunition,
    exploration_guild_reputation::ExplorationGuildReputation, fleet_credits::FleetCredits,
    mine_ammunition::MineAmmunition, selected_weapon::SelectedWeapon,
    space_zone_border::SpaceZoneBorder, torpedo_ammunition::TorpedoAmmunition,
    trading_guild_reputation::TradingGuildReputation,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceZoneBorder {
            top_border: 240.0,
            left_border: -330.0,
            bottom_border: -240.0,
            right_border: 330.0,
        })
        .insert_resource(FleetCredits(1000))
        .insert_resource(SelectedWeapon(1))
        .insert_resource(MineAmmunition(7))
        .insert_resource(TorpedoAmmunition(5))
        .insert_resource(BlasterAmmunition(20))
        .insert_resource(ExoticAmmunition(2))
        .insert_resource(CombatGuildReputation(1))
        .insert_resource(ExplorationGuildReputation(1))
        .insert_resource(TradingGuildReputation(1))
        .insert_resource(CameraSettings {
            zoom_speed: 1.5,
            min_zoom: 0.1,
            max_zoom: 2.,
            current_zoom: 1.,
        })
        .insert_resource(Chunks { chunks: Vec::new() });
    }
}
