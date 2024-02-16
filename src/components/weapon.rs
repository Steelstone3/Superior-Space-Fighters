use bevy::{ecs::component::Component, math::Vec2, time::Timer};

#[derive(Component)]
pub struct Weapon {
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}
