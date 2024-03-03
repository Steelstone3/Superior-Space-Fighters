use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct TargetingSettings {
    pub max_distance: f32,
    pub auto_target_max_distance: f32,
}
