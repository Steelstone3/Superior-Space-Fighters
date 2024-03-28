use crate::components::{
    starships::starship::Starship, weapons::player_weapons::player_torpedo::PlayerTorpedo,
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
pub struct MutablePlayerTorpedoEntityTransformQuery {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub player_torpedo: &'static mut PlayerTorpedo,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerTorpedoTransformQuery {
    pub transform: &'static mut Transform,
    pub player_torpedo: &'static PlayerTorpedo,
}

#[derive(QueryFilter)]
pub struct PlayerTorpedoFilter {
    with_player_torpedo: With<PlayerTorpedo>,
    without_starship: Without<Starship>,
}
