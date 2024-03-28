use crate::{
    queries::camera_queries::MutableCameraOrthographicProjectionQuery,
    resources::camera_settings::CameraSettings,
};
use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, ResMut},
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
};
use float_lerp::lerp;

pub fn scroll_camera(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut cameras: Query<MutableCameraOrthographicProjectionQuery>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let mut camera = cameras.single_mut();

    for event in scroll_event_reader.read() {
        match event.unit {
            // For mouse
            MouseScrollUnit::Line => {
                if event.y < 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom
                        * camera_settings.zoom_in
                        * camera_settings.zoom_speed)
                        .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
                } else if event.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom
                        * camera_settings.zoom_out
                        / camera_settings.zoom_speed)
                        .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
                }
            }
            // For touchpads
            MouseScrollUnit::Pixel => {
                if event.y < 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom
                        * camera_settings.zoom_in
                        * camera_settings.zoom_speed)
                        .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
                } else if event.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom
                        * camera_settings.zoom_out
                        / camera_settings.zoom_speed)
                        .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
                }
            }
        }
    }
    camera.projection.scale = lerp(camera.projection.scale, camera_settings.current_zoom, 0.1);
}
