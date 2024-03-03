use bevy::ecs::query::{QueryFilter, Without};

use crate::components::{
    player_starship::PlayerStarship, starship::Starship, weapons::target_arrow::TargetArrow,
};

#[derive(QueryFilter)]
pub struct TargetFilter {
    without_a: Without<Starship>,
    without_b: Without<PlayerStarship>,
    without_c: Without<TargetArrow>,
}
