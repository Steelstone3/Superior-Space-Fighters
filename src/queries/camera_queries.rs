use bevy::{
    ecs::query::{QueryData, QueryFilter, With, Without},
    render::camera::{Camera, OrthographicProjection},
    transform::components::Transform,
};

use crate::components::starships::player_starship::PlayerStarship;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct CameraMutableTransformQuery {
    pub transform: &'static mut Transform,
    pub camera: &'static Camera,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct CameraMutableOrthographicProjectionQuery {
    pub projection: &'static mut OrthographicProjection,
    pub camera: &'static Camera,
}

#[derive(QueryFilter)]
pub struct CameraFilter {
    with_camera: With<Camera>,
    without_player_starship: Without<PlayerStarship>,
}
