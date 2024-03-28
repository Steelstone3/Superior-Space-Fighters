use crate::components::starships::starship::Starship;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
pub struct StarshipTransformQuery {
    pub transform: &'static Transform,
    pub starship: &'static Starship,
}
