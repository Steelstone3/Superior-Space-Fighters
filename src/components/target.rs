use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct Target {
    pub target_entity: Entity,
}
