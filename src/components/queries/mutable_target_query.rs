use crate::components::weapons::weapon_types::combat_target::CombatTarget;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableTargetQuery {
    pub transform: &'static mut Transform,
    pub target: &'static CombatTarget,
}
