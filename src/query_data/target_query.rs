use bevy::{
    ecs::{entity::Entity, query::QueryData},
    transform::components::Transform,
};

use crate::components::weapons::target::Target;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct TargetQuery {
    pub transform: &'static mut Transform,
    pub target: &'static Target,
    pub entity: Entity,
}
