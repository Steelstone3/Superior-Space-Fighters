use bevy::ecs::{entity::Entity, query::QueryData};

#[derive(QueryData)]
pub struct EntityQuery {
    pub entity: Entity,
}
