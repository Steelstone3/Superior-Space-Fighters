use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::exotics::ExoticSprite,
    sounds::starships::weapons::exotics::ExoticSound,
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub sound: ExoticSound,
    pub weapon: Weapon,
}
