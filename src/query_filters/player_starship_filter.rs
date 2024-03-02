use bevy::ecs::query::{QueryFilter, With, Without};

use crate::components::{player_starship::PlayerStarship, starship::Starship};

#[derive(QueryFilter)]
pub struct PlayerStarshipFilter {
    with_a: With<PlayerStarship>,
    without_b: Without<Starship>,
}
