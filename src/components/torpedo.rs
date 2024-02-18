use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::torpedos::TorpedoSprite,
    sounds::starships::weapons::{impacts::ImpactSound, torpedos::TorpedoSound},
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Torpedo {
    pub torpedo: TorpedoSprite,
    pub firing_sound: TorpedoSound,
    pub impact_sound: ImpactSound,
    pub weapon: Weapon,
}
