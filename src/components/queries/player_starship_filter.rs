use bevy::ecs::query::{QueryFilter, With, Without};

use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
};

#[derive(QueryFilter)]
pub struct PlayerStarshipFilter {
    with_player_starship: With<PlayerStarship>,
    without_starship: Without<Starship>,
    without_target: Without<Target>,
}
