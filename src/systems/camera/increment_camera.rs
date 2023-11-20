use crate::resources::camera_settings::CameraSettings;
use bevy::prelude::{Camera, Input, KeyCode, OrthographicProjection, Query, Res, ResMut, With};

pub fn increment_camera(
    _input: Res<Input<KeyCode>>,
    _camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    _camera_settings: ResMut<CameraSettings>,
) {
}
