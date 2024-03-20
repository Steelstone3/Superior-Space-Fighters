use bevy::{ecs::system::Resource, time::Timer};

#[derive(Resource)]
pub struct ProjectileFireRate {
    pub blaster_fire_rate: Timer,
    pub torpedo_fire_rate: Timer,
    pub mine_fire_rate: Timer,
    pub exotic_fire_rate: Timer,
}
