use crate::components::starships::player_starship::PlayerStarship;
use bevy::{
    ecs::query::{QueryFilter, With, Without},
    render::camera::Camera,
};

#[derive(QueryFilter)]
pub struct CameraFilter {
    with_camera: With<Camera>,
    without_player_starship: Without<PlayerStarship>,
}
