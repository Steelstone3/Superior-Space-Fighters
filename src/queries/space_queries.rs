use crate::components::{space::Space, starships::player_starship::PlayerStarship};
use bevy::{
    ecs::query::{QueryData, QueryFilter, With, Without},
    transform::components::Transform,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableSpaceTransformQuery {
    pub transform: &'static mut Transform,
    pub space: &'static mut Space,
}

#[derive(QueryFilter)]
pub struct SpaceFilter {
    with_space: With<Space>,
    without_player_starship: Without<PlayerStarship>,
}
