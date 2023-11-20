use crate::resources::camera_settings::CameraSettings;
use bevy::{
    prelude::{Camera, Input, KeyCode, OrthographicProjection, Query, Res, ResMut, With},
    utils::tracing,
};
use float_lerp::lerp;

pub fn increment_camera(
    input: Res<Input<KeyCode>>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let mut camera_projection = camera_projection_query.single_mut();

    if input.just_pressed(KeyCode::Equals) {
        camera_settings.current_zoom = (camera_settings.current_zoom * 0.9
            / camera_settings.zoom_speed)
            .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
        tracing::info!("Camera zoomed in");
    }
    if input.just_pressed(KeyCode::Minus) {
        camera_settings.current_zoom =
            (camera_settings.current_zoom * 1.1 * camera_settings.zoom_speed)
                .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
        tracing::info!("Camera zoomed out");
    }
    if input.just_pressed(KeyCode::Home) {
        camera_settings.current_zoom = 1.0;
        tracing::info!("Camera zoom reset")
    }

    camera_projection.scale = lerp(camera_projection.scale, camera_settings.current_zoom, 0.1);
}
