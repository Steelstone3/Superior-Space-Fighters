use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};

#[derive(Component)]
pub struct Weapon {
    pub original_position: Vec3,
    pub velocity: f32,
    pub size: Vec2,
    pub range: f32,
}
