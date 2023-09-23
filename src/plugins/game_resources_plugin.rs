use crate::resources::{
    mine_ammunition::MineAmmunition, combat_guild_reputation::CombatGuildReputation,
    exploration_guild_reputation::ExplorationGuildReputation, fleet_credits::FleetCredits,
    trading_guild_reputation::TradingGuildReputation, torpedo_ammunition::TorpedoAmmunition,
};
use bevy::prelude::{App, Plugin};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FleetCredits(1000.0))
            .insert_resource(MineAmmunition(100.0))
            .insert_resource(TorpedoAmmunition(100.0))
            .insert_resource(CombatGuildReputation(1.0))
            .insert_resource(ExplorationGuildReputation(1.0))
            .insert_resource(TradingGuildReputation(1.0));
    }
}
