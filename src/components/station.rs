use bevy::{ecs::component::Component, prelude::Vec2};

use crate::assets::images::space_stations::SpaceStationSprite;

#[derive(Component)]
pub struct Station {
    pub station: SpaceStationSprite,
    pub size: Vec2,
}
