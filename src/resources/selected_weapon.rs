use bevy::{
    ecs::{reflect::ReflectResource, system::Resource},
    reflect::Reflect,
};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SelectedWeaponResource {
    pub selected_weapon: u32,
}

pub enum SelectedWeaponEnum {
    Blaster = 1,
    Torpedo = 2,
    Mine = 3,
    Exotic = 4,
}
