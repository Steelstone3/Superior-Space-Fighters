use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SelectedWeaponResource {
    pub selected_weapon: u32,
}

pub enum SelectedWeaponEnum {
    Blaster = 1,
    Torpedo = 2,
    Mine = 3,
    Exotic = 4,
}
