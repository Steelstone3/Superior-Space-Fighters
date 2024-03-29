use bevy::{
    ecs::{entity::Entity, event::Event},
    math::{Quat, Vec2, Vec3},
};

#[derive(Event)]
pub struct SpawnSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub translation: Vec3,
    pub entity: Entity,
    pub rotation: Quat,
}
