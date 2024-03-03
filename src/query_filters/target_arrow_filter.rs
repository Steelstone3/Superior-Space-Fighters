use bevy::ecs::query::{QueryFilter, Without};

use crate::components::{
    player_starship::PlayerStarship, starship::Starship, weapons::target::Target,
};

#[derive(QueryFilter)]
pub struct TargetArrowFilter {
    without_a: Without<Starship>,
    without_b: Without<PlayerStarship>,
    without_c: Without<Target>,
}
