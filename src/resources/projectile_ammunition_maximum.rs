use bevy::ecs::system::Resource;

#[derive(Resource)]
#[allow(dead_code)]
pub struct ProjectileAmmunitionMaximum {
    pub maximum_blaster_ammunition: u32,
    pub maximum_torpedo_ammunition: u32,
    pub maximum_mine_ammunition: u32,
    pub maximum_exotic_ammunition: u32,
}
