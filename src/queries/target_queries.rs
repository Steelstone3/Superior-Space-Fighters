use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
};
use bevy::{
    ecs::{
        entity::Entity,
        query::{QueryData, QueryFilter, With, Without},
    },
    transform::components::Transform,
};

#[derive(QueryData)]
pub struct TargetEntityQuery {
    pub entity: Entity,
    pub target: &'static Target,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct TargetMutableTransformQuery {
    pub transform: &'static mut Transform,
    pub target: &'static Target,
}

#[derive(QueryFilter)]
pub struct TargetFilter {
    with_target: With<Target>,
    without_player_starship: Without<PlayerStarship>,
    without_starship: Without<Starship>,
}
