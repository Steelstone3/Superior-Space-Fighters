use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct WeaponSelection {
    pub selected_weapon: u32,
}
