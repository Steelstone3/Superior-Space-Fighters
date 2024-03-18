use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::targetting::target::Target;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableTargetQuery {
    pub transform: &'static mut Transform,
    pub target: &'static Target,
}
