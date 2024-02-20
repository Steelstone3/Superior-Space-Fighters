use super::mine::Mine;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct PlayerMine {
    pub mine: Mine,
}
