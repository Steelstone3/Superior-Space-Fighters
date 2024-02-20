use crate::assets::{
    images::starships::weapons::mines::MineSprite,
    sounds::starships::weapons::{impacts::ImpactSound, mines::MineSound},
};
use bevy::{ecs::component::Component, math::Vec2, time::Timer};

#[derive(Component)]
pub struct Mine {
    pub mine: MineSprite,
    pub firing_sound: MineSound,
    pub impact_sound: ImpactSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
