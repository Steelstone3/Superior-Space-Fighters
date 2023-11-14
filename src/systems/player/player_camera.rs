use bevy::{core_pipeline::core_2d::Camera2dBundle, ecs::{system::{Commands, Query, ResMut, ParamSet}, query::With, event::EventReader}, transform::components::Transform, render::camera::{Camera, OrthographicProjection, self}, input::{keyboard::KeyCode, mouse::{MouseScrollUnit, MouseWheel}, Input}, math::Vec3};
use float_lerp::lerp;

use crate::{resources::camera_settings::CameraSettings, components::player_starship::PlayerStarship};

pub fn add_player_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_movement(
    mut scroll_evr: EventReader<MouseWheel>,
    mut trans_set: ParamSet<(Query<&mut Transform, With<Camera>>, Query<&Transform, With<PlayerStarship>>)>,
    mut camera_proj_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
)
{
    let mut camera_proj = camera_proj_query.single_mut();
    let mut player_tran: Vec3 = Default::default();
    for player_transform in trans_set.p1().iter_mut(){
        player_tran = player_transform.translation;
    }

    for mut camera_tran in trans_set.p0().iter_mut(){
        camera_tran.translation = player_tran;
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