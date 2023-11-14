use bevy::{
    ecs::{
        event::EventReader,
        query::With,
        system::{Query, ResMut},
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    render::camera::{Camera, OrthographicProjection},
};
use float_lerp::lerp;

use crate::resources::camera_settings::CameraSettings;

pub fn scroll_camera(
    mut scroll_event_reader: EventReader<MouseWheel>,
    // mut transform_set: ParamSet<(
    //     Query<&mut Transform, With<Camera>>,
    //     Query<&Transform, With<PlayerStarship>>,
    // )>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let mut camera_projection = camera_projection_query.single_mut();
    // let mut player_transform: Vec3 = Default::default();
    // for player_t in transform_set.p1().iter_mut() {
    //     player_transform = player_t.translation;
    // }

    // for mut camera_transform in transform_set.p0().iter_mut() {
    //     camera_transform.translation = player_transform;
    // }

    for event in scroll_event_reader.read() {
        match event.unit {
            //For mouse
            MouseScrollUnit::Line => {
                if event.y < 0.0 {
                    camera_settings.current_zoom =
                        (camera_settings.current_zoom * 1.25 * camera_settings.zoom_speed)
                            .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                } else if event.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 0.75
                        / camera_settings.zoom_speed)
                        .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }
            }
            //For touchpads
            MouseScrollUnit::Pixel => {
                if event.y < 0.0 {
                    camera_settings.current_zoom =
                        (camera_settings.current_zoom * 1.25 * camera_settings.zoom_speed)
                            .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                } else if event.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 0.75
                        / camera_settings.zoom_speed)
                        .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }
            }
        }
    }
    camera_projection.scale = lerp(camera_projection.scale, camera_settings.current_zoom, 0.1);
}
