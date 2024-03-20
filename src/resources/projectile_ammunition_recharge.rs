use bevy::{ecs::system::Resource, time::Timer};

#[derive(Resource)]
pub struct ProjectileAmmunitionRecovery {
    pub blaster_recovery_rate: Timer,
    pub torpedo_recovery_rate: Timer,
    pub mine_recovery_rate: Timer,
    pub exotic_recovery_rate: Timer,
}
