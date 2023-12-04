use crate::assets::{
    images::starships::weapons::torpedos::TorpedoSprite,
    sounds::starships::weapons::torpedos::TorpedoSound,
};
use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

#[derive(Component)]
pub struct Torpedo {
    pub torpedo: TorpedoSprite,
    pub sound: TorpedoSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
