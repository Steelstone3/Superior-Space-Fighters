use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::images::weapons::torpedos::TorpedoSprite;

#[derive(Component)]
pub struct Torpedo {
    pub torpedo: TorpedoSprite,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
