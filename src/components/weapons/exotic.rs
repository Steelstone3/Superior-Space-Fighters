use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::exotics::ExoticSprite,
    sounds::starships::weapons::{exotics::ExoticSound, impacts::ImpactSound},
};
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub firing_sound: ExoticSound,
    pub impact_sound: ImpactSound,
    pub weapon: Weapon,
}

impl Exotic {
    pub fn new(original_position: Vec3) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod exotic_should {
    use bevy::math::{Vec2, Vec3};
    use super::*;
    
    #[test]
    fn create_new() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_blaster = Exotic {
            exotic: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            weapon: Weapon {
                original_position,
                velocity: 100.0,
                size: Vec2 { x: 100.0, y: 100.0 },
                range: 750.0,
            },
        };

        // When
        let blaster = Exotic::new(original_position);

        // Then
        assert_eq!(expected_blaster, blaster)
    }
}
