use bevy::{ecs::query::QueryData, render::camera::Camera, transform::components::Transform};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableCameraTransformQuery {
    pub transform: &'static mut Transform,
    pub camera: &'static Camera,
}
