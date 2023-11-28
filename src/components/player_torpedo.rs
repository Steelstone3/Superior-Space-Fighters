use super::torpedo::Torpedo;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct PlayerTorpedo {
    pub torpedo: Torpedo,
}
