use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct ProjectileAmmunition {
    pub blaster_ammunition: u32,
    pub torpedo_ammunition: u32,
    pub mine_ammunition: u32,
    pub exotic_ammunition: u32,
    pub selected_weapon: u32,
}
