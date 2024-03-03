use bevy::{
    ecs::{entity::Entity, query::QueryData},
    transform::components::Transform,
};

use crate::components::weapons::target_arrow::TargetArrow;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct TargetArrowQuery {
    pub transform: &'static mut Transform,
    pub target: &'static TargetArrow,
    pub entity: Entity,
}
