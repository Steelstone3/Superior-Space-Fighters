use crate::components::{
    starships::starship::Starship, weapons::player_weapons::player_blaster::PlayerBlaster,
};
use bevy::{
    ecs::{
        entity::Entity,
        query::{QueryData, QueryFilter, With, Without},
    },
    transform::components::Transform,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerBlasterEntityTransformQuery {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub player_blaster: &'static mut PlayerBlaster,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerBlasterTransformQuery {
    pub transform: &'static mut Transform,
    pub player_blaster: &'static mut PlayerBlaster,
}

#[derive(QueryFilter)]
pub struct PlayerBlasterFilter {
    with_player_blaster: With<PlayerBlaster>,
    without_starship: Without<Starship>,
}
