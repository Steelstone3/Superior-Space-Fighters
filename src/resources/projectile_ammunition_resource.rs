use bevy::{
    ecs::{reflect::ReflectResource, system::Resource},
    reflect::Reflect,
};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ProjectileAmmunitionResource {
    pub blaster_ammunition: u32,
    pub torpedo_ammunition: u32,
    pub mine_ammunition: u32,
    pub exotic_ammunition: u32,
    pub maximum_blaster_ammunition: u32,
    pub maximum_torpedo_ammunition: u32,
    pub maximum_mine_ammunition: u32,
    pub maximum_exotic_ammunition: u32,
}
