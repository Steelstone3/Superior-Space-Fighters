use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
};
use bevy::{
    ecs::query::{QueryData, QueryFilter, With, Without},
    transform::components::Transform,
};

#[derive(QueryData)]
pub struct StarshipTransformQuery {
    pub transform: &'static Transform,
    pub starship: &'static Starship,
}

#[derive(QueryFilter)]
pub struct StarshipFilter {
    with_starship: With<Starship>,
    without_player_starship: Without<PlayerStarship>,
    without_target: Without<Target>,
}
