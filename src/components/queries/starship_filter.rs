use bevy::ecs::query::{QueryFilter, With, Without};

use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship}, targetting::target::Target,
};

#[derive(QueryFilter)]
pub struct StarshipFilter {
    with_starship: With<Starship>,
    without_player_starship: Without<PlayerStarship>,
    without_target: Without<Target>,
}
