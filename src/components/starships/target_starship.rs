use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component, Debug, PartialEq)]
pub struct TargetStarship {
    pub starship_target: Entity,
}
