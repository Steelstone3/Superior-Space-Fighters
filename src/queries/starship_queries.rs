use crate::components::{
    starships::{ai_starship::AIStarship, player_starship::PlayerStarship, starship::Starship},
    user_interface::targetting::target::Target,
    weapons::player_weapons::{
        player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
        player_torpedo::PlayerTorpedo,
    },
};
use bevy::{
    ecs::{
        entity::Entity,
        query::{QueryData, QueryFilter, With, Without},
    },
    transform::components::Transform,
};

#[derive(QueryData)]
pub struct AIStarshipQuery {
    pub starship: &'static Starship,
    pub ai_starship: &'static AIStarship,
}

#[derive(QueryData)]
pub struct AIStarshipEntityQuery {
    pub entity: Entity,
    pub starship: &'static Starship,
    pub ai_starship: &'static AIStarship,
}

#[derive(QueryData)]
pub struct AIStarshipTransformQuery {
    pub transform: &'static Transform,
    pub starship: &'static Starship,
    pub ai_starship: &'static AIStarship,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableAIStarshipTransformQuery {
    pub transform: &'static mut Transform,
    pub starship: &'static mut Starship,
    pub ai: &'static mut AIStarship,
}

#[derive(QueryFilter)]
pub struct AIStarshipFilter {
    with_starship: With<Starship>,
    with_ai_starship: With<AIStarship>,
    without_player_starship: Without<PlayerStarship>,
    without_target: Without<Target>,
    without_player_blaster: Without<PlayerBlaster>,
    without_player_torpedo: Without<PlayerTorpedo>,
    without_player_mine: Without<PlayerMine>,
    without_player_exotic: Without<PlayerExotic>,
}
