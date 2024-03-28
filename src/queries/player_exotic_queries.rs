use crate::components::{
    starships::starship::Starship, weapons::player_weapons::player_exotic::PlayerExotic,
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
pub struct MutablePlayerExoticEntityTransformQuery {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub player_exotic: &'static mut PlayerExotic,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerExoticTransformQuery {
    pub transform: &'static mut Transform,
    pub player_exotic: &'static PlayerExotic,
}

#[derive(QueryFilter)]
pub struct PlayerExoticFilter {
    with_player_exotic: With<PlayerExotic>,
    without_starship: Without<Starship>,
}
