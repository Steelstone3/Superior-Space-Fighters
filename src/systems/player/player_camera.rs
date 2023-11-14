use bevy::{core_pipeline::core_2d::Camera2dBundle, ecs::{system::{Commands, Query, ResMut, Res}, query::With, event::EventReader}, transform::components::Transform, render::camera::{Camera, OrthographicProjection}, input::{keyboard::KeyCode, mouse::{MouseScrollUnit, MouseWheel}, Input}};
use float_lerp::lerp;

use crate::resources::camera_settings::CameraSettings;

pub fn add_player_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_movement(
    keys: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_trans_query: Query<&mut Transform, With<Camera>>,
    mut camera_proj_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
)
{
    let mut camera_proj = camera_proj_query.single_mut();
    let mut camera_transform = camera_trans_query.single_mut();

    if keys.pressed(KeyCode::Up) {
        camera_transform.translation.y += camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Down) {
        camera_transform.translation.y -= camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += camera_settings.camera_speed * camera_proj.scale * 2.0;
    }

    for ev in scroll_evr.read() {
        match ev.unit {
            //For mouse
            MouseScrollUnit::Line => {
                if ev.y < 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 1.25 * camera_settings.zoom_speed).clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }else if ev.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 0.75 / camera_settings.zoom_speed).clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }
            }
            //For touchpads
            MouseScrollUnit::Pixel => {
                if ev.y < 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 1.25 * camera_settings.zoom_speed).clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }else if ev.y > 0.0 {
                    camera_settings.current_zoom = (camera_settings.current_zoom * 0.75 / camera_settings.zoom_speed).clamp(camera_settings.min_zoom, camera_settings.max_zoom);
                }
            }
        }
    }

    camera_proj.scale = lerp(camera_proj.scale, camera_settings.current_zoom, 0.1);
}