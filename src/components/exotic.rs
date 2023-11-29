use crate::assets::{
    images::starships::weapons::exotics::ExoticSprite,
    sounds::starships::weapons::exotics::ExoticSound,
};
use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

#[derive(Component)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub sound: ExoticSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
