use crate::resources::{
    blaster_ammunition::BlasterAmmunition, combat_guild_reputation::CombatGuildReputation,
    exotic_ammunition::ExoticAmmunition, exploration_guild_reputation::ExplorationGuildReputation,
    fleet_credits::FleetCredits, mine_ammunition::MineAmmunition, selected_weapon::SelectedWeapon,
    torpedo_ammunition::TorpedoAmmunition, trading_guild_reputation::TradingGuildReputation,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FleetCredits(1000))
            .insert_resource(SelectedWeapon(1))
            .insert_resource(MineAmmunition(7))
            .insert_resource(TorpedoAmmunition(5))
            .insert_resource(BlasterAmmunition(20))
            .insert_resource(ExoticAmmunition(2))
            .insert_resource(CombatGuildReputation(1))
            .insert_resource(ExplorationGuildReputation(1))
            .insert_resource(TradingGuildReputation(1));
    }
}
