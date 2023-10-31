use bevy::{ecs::component::Component, prelude::Vec2};

use crate::assets::images::space_ships::SpaceShipSprite;

#[derive(Component)]
pub struct Starship {
    pub asset: SpaceShipSprite,
    pub acceleration: f32,
    pub current_velocity: f32,
    pub velocity: f32,
    pub rotation: f32,
    pub size: Vec2,
}
