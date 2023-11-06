use bevy::{ecs::component::Component, prelude::Vec2};

use crate::assets::images::planets::PlanetSprite;

#[derive(Component)]
pub struct Planet {
    pub planet: PlanetSprite,
    pub size: Vec2,
}
