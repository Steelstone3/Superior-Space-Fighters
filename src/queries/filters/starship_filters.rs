use bevy::ecs::query::{QueryFilter, With, Without};

use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
};

#[derive(QueryFilter)]
pub struct StarshipWithoutPlayerStarshipAndTargetFilter {
    with_starship: With<Starship>,
    without_player_starship: Without<PlayerStarship>,
    without_target: Without<Target>,
}
