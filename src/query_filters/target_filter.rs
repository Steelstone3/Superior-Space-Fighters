use bevy::ecs::query::{QueryFilter, Without};

use crate::components::{player_starship::PlayerStarship, starship::Starship};

#[derive(QueryFilter)]
pub struct TargetFilter {
    without_a: Without<Starship>,
    without_b: Without<PlayerStarship>,
}
