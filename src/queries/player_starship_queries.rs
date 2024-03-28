use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
};
use bevy::{
    ecs::query::{QueryData, QueryFilter, With, Without},
    transform::components::Transform,
};

#[derive(QueryData)]
pub struct PlayerStarshipTransformQuery {
    pub transform: &'static Transform,
    pub player_starship: &'static PlayerStarship,
}

#[derive(QueryFilter)]
pub struct PlayerStarshipFilter {
    with_player_starship: With<PlayerStarship>,
    without_starship: Without<Starship>,
    without_target: Without<Target>,
}
