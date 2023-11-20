use crate::resources::camera_settings::CameraSettings;
use bevy::{
    prelude::{Camera, Input, KeyCode, OrthographicProjection, Query, Res, ResMut, With},
    utils::tracing,
};

pub fn increment_camera(
    input: Res<Input<KeyCode>>,
    _camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    _camera_settings: ResMut<CameraSettings>,
) {
    if input.just_pressed(KeyCode::Equals) {
        tracing::info!("Camera zoomed in");
    }
    if input.just_pressed(KeyCode::Minus) {
        tracing::info!("Camera zoomed out");
    }
    if input.just_pressed(KeyCode::Key0) {
        tracing::info!("Camera zoom reset")
    }
}
