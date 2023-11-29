use crate::assets::{
    images::starships::weapons::blasters::BlasterSprite,
    sounds::starships::weapons::blasters::BlasterSound,
};
use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

#[derive(Component)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub sound: BlasterSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
