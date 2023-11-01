use super::starship::Starship;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct PlayerStarship {
    pub ship: Starship,
}
