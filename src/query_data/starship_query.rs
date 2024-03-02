use bevy::{
    ecs::{entity::Entity, query::QueryData},
    transform::components::Transform,
};

use crate::components::starship::Starship;

#[derive(QueryData)]
pub struct StarshipQuery {
    pub transform: &'static Transform,
    pub starship: &'static Starship,
    pub entity: Entity,
}
