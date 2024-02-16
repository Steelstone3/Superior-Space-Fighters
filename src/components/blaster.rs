use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::blasters::BlasterSprite,
    sounds::starships::weapons::blasters::BlasterSound,
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub sound: BlasterSound,
    pub weapon: Weapon,
}
