use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct CombatGuildReputation(pub u32);
