use crate::assets::images::starships::factions::SpaceShipSprite;
use bevy::{ecs::component::Component, prelude::Vec2, transform::components::Transform};

#[derive(Component)]
pub struct Starship {
    pub ship: SpaceShipSprite,
    pub acceleration: f32,
    pub current_velocity: f32,
    pub velocity: f32,
    pub rotation: f32,
    pub transform: Transform,
    pub size: Vec2,
}
