use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::images::weapons::exotics::ExoticSprite;

#[derive(Component)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub speed: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
