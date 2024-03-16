use bevy::ecs::query::{QueryFilter, With, Without};

use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    weapons::weapon_types::target::Target,
};

#[derive(QueryFilter)]
pub struct TargetFilter {
    with_target: With<Target>,
    without_player_starship: Without<PlayerStarship>,
    without_starship: Without<Starship>,
}
