use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::exotics::ExoticSprite,
    sounds::starships::weapons::{exotics::ExoticSound, impacts::ImpactSound},
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub firing_sound: ExoticSound,
    pub impact_sound: ImpactSound,
    pub weapon: Weapon,
}
