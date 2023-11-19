use super::exotic::Exotic;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct PlayerExotic {
    pub exotic: Exotic,
}
