use crate::{
    queries::camera_queries::MutableCameraOrthographicProjectionQuery,
    resources::camera_settings::CameraSettings,
};
use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, ResMut},
    utils::tracing,
};
use float_lerp::lerp;

pub fn increment_camera(
    input: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<MutableCameraOrthographicProjectionQuery>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let mut camera = cameras.single_mut();

    if input.just_pressed(KeyCode::Equal) {
        camera_settings.current_zoom = (camera_settings.current_zoom * camera_settings.zoom_out
            / camera_settings.zoom_speed)
            .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        tracing::info!("Camera zoomed in");
    }
    if input.just_pressed(KeyCode::Minus) {
        camera_settings.current_zoom =
            (camera_settings.current_zoom * camera_settings.zoom_out * camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        tracing::info!("Camera zoomed out");
    }
    if input.just_pressed(KeyCode::Home) {
        camera_settings.current_zoom = 1.0;
        tracing::info!("Camera zoom reset")
    }

    camera.projection.scale = lerp(camera.projection.scale, camera_settings.current_zoom, 0.1);
}
