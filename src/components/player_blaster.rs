use super::blaster::Blaster;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct PlayerBlaster {
    pub blaster: Blaster,
}
