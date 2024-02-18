use crate::assets::{
    images::starships::faction_starships::FactionStarshipSprite,
    sounds::starships::engines::EngineSound,
};
use bevy::{ecs::component::Component, prelude::Vec2, transform::components::Transform};

#[derive(Component)]
pub struct Starship {
    pub faction_starship: FactionStarshipSprite,
    pub engine: EngineSound,
    pub acceleration: f32,
    pub current_velocity: f32,
    pub velocity: f32,
    pub rotation: f32,
    pub transform: Transform,
    pub size: Vec2,
}
