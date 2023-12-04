use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct CameraSettings {
    pub zoom_speed: f32,
    pub minimum_zoom: f32,
    pub maximum_zoom: f32,
    pub current_zoom: f32,
    pub zoom_in: f32,
    pub zoom_out: f32,
}
