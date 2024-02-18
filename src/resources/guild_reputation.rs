use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GuildReputation {
    pub trading_guild_reputation: u32,
    pub exploration_guild_reputation: u32,
    pub combat_guild_reputation: u32,
}
