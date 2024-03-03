use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct TargetArrow {
    pub target_entity: Entity,
}
