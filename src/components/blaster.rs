use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::{
    images::weapons::blasters::BlasterSprite, sounds::weapons::blasters::BlasterSound,
};

#[derive(Component)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub sound: BlasterSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
