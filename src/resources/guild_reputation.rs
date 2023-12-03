use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GuildReputation {
    trading_guild_reputation: u32,
    exploration_guild_reputation: u32,
    combat_guild_reputation: u32,
}
