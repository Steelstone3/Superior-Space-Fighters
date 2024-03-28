use bevy::{
    ecs::event::Event,
    math::{Vec2, Vec3},
};

use crate::components::planet::Planet;

#[derive(Event)]
pub struct SpawnPlanetSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub translation: Vec3,
    pub planet: Planet,
}
