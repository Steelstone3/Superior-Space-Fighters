use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::blasters::BlasterSprite,
    sounds::starships::weapons::{blasters::BlasterSound, impacts::ImpactSound},
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub firing_sound: BlasterSound,
    pub impact_sound: ImpactSound,
    pub weapon: Weapon,
}
