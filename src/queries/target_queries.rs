use crate::components::user_interface::targetting::target::Target;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableTargetTransformQuery {
    pub transform: &'static mut Transform,
    pub target: &'static Target,
}
