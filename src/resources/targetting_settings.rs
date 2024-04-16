use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct TargettingSettingsResource {
    pub maximum_distance: f32,
    pub is_targetting: bool,
}
