use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct DespawnSpriteEvent {
    pub entity: Entity,
}
