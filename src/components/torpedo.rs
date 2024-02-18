use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::torpedos::TorpedoSprite,
    sounds::starships::weapons::torpedos::TorpedoSound,
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Torpedo {
    pub torpedo: TorpedoSprite,
    pub sound: TorpedoSound,
    pub weapon: Weapon,
}
