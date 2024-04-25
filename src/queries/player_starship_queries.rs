use crate::components::{
    space::Space,
    starships::{ai_starship::AIStarship, player_starship::PlayerStarship, starship::Starship},
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

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutablePlayerStarshipTransformQuery {
    pub transform: &'static mut Transform,
    pub player_starship: &'static mut PlayerStarship,
    pub starship: &'static mut Starship,
}

#[derive(QueryFilter)]
pub struct PlayerStarshipFilter {
    with_player_starship: With<PlayerStarship>,
    without_starship: Without<AIStarship>,
    without_target: Without<Target>,
    without_space: Without<Space>,
}
