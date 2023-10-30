use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct TradingGuildReputation(pub u32);
