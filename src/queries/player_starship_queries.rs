use crate::components::starships::player_starship::PlayerStarship;
use bevy::{ecs::query::QueryData, transform::components::Transform};

#[derive(QueryData)]
pub struct PlayerStarshipTransformQuery {
    pub transform: &'static Transform,
    pub player_starship: &'static PlayerStarship,
}
