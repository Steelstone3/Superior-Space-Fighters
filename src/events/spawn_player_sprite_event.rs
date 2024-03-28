use bevy::{
    ecs::event::Event,
    math::{Vec2, Vec3},
};

use crate::components::starships::player_starship::PlayerStarship;

#[derive(Event)]
pub struct SpawnPlayerSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub translation: Vec3,
    pub player: PlayerStarship,
}
