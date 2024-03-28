use bevy::{
    ecs::event::Event,
    math::{Quat, Vec2, Vec3},
};

use crate::components::starships::starship::Starship;

#[derive(Event)]
pub struct SpawnStarshipSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub translation: Vec3,
    pub ship: Starship,
    pub rotation: Quat,
}
