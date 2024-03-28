use crate::components::{
    starships::starship::Starship, weapons::player_weapons::player_mine::PlayerMine,
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
pub struct MutablePlayerMineEntityTransformQuery {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub player_mine: &'static mut PlayerMine,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerMineEntityQuery {
    pub entity: Entity,
    pub player_mine: &'static mut PlayerMine,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerMineTransformQuery {
    pub transform: &'static mut Transform,
    pub player_mine: &'static PlayerMine,
}

#[derive(QueryFilter)]
pub struct PlayerMineFilter {
    with_player_mine: With<PlayerMine>,
    without_starship: Without<Starship>,
}
