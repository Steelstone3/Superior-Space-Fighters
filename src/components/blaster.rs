use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::images::weapons::blasters::BlasterSprite;

#[derive(Component)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub speed: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
