use bevy::{ecs::component::Component, prelude::Vec2, time::Timer};

use crate::assets::images::weapons::mines::MineSprite;

#[derive(Component)]
pub struct Mine {
    pub mine: MineSprite,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
