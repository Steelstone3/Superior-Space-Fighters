use bevy::{ecs::component::Component, prelude::Vec2};

use crate::assets::images::space_ships::SpaceShipSprite;

#[derive(Component)]
pub struct Player {
    pub ship: SpaceShipSprite,
    pub speed: f32,
    pub size: Vec2,
}
