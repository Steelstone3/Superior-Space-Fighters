use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::{
    images::weapons::exotics::ExoticSprite, sounds::weapons::exotics::ExoticSound,
};

#[derive(Component)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub sound: ExoticSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
