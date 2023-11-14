use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct CameraSettings {
    pub camera_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub current_zoom: f32,
}
