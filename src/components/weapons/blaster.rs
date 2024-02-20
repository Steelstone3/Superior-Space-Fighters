use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::blasters::BlasterSprite,
    sounds::starships::weapons::{blasters::BlasterSound, impacts::ImpactSound},
};
use bevy::{
    ecs::component::Component,
    math::Vec3,
};

#[derive(Component, Debug, PartialEq)]
pub struct Blaster {
    pub blaster: BlasterSprite,
    pub firing_sound: BlasterSound,
    pub impact_sound: ImpactSound,
    pub weapon: Weapon,
}

impl Blaster {
    pub fn new(original_position: Vec3) -> Self {
        Self {
            blaster: BlasterSprite::default(),
            firing_sound: BlasterSound::default(),
            impact_sound: ImpactSound::default(),
            weapon: Weapon::new(original_position, 100.0, 100.0, 750.0),
        }
    }
}

#[cfg(test)]
mod blaster_should {
    use bevy::math::Vec2;
    use super::*;
    
    #[test]
    fn create_new() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_blaster = Blaster {
            blaster: Default::default(),
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
        let blaster = Blaster::new(original_position);

        // Then
        assert_eq!(expected_blaster, blaster)
    }
}
