use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
    weapons::player_weapons::{
        player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
        player_torpedo::PlayerTorpedo,
    },
};
use bevy::{
    ecs::{
        entity::Entity,
        query::{QueryData, QueryFilter, With, Without},
    },
    transform::components::Transform,
};

#[derive(QueryData)]
pub struct StarshipQuery {
    pub starship: &'static Starship,
}

#[derive(QueryData)]
pub struct StarshipEntityQuery {
    pub entity: Entity,
    pub starship: &'static Starship,
}

#[derive(QueryData)]
pub struct StarshipTransformQuery {
    pub transform: &'static Transform,
    pub starship: &'static Starship,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableStarshipEntityTransformQuery {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub starship: &'static mut Starship,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableStarshipTransformQuery {
    pub transform: &'static mut Transform,
    pub starship: &'static mut Starship,
}

#[derive(QueryFilter)]
pub struct StarshipFilter {
    with_starship: With<Starship>,
    without_player_starship: Without<PlayerStarship>,
    without_target: Without<Target>,
    without_player_blaster: Without<PlayerBlaster>,
    without_player_torpedo: Without<PlayerTorpedo>,
    without_player_mine: Without<PlayerMine>,
    without_player_exotic: Without<PlayerExotic>,
}
