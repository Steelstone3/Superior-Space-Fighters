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
        Self {
            exotic: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            weapon: Weapon::new(original_position, 80.0, 75.0, 500.0),
        }
    }
}

#[cfg(test)]
mod exotic_should {
    use super::*;
    use bevy::math::{Vec2, Vec3};

    #[test]
    fn create_new() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_exotic = Exotic {
            exotic: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            weapon: Weapon {
                original_position,
                velocity: 75.0,
                size: Vec2 { x: 80.0, y: 80.0 },
                range: 500.0,
            },
        };

        // When
        let exotic = Exotic::new(original_position);

        // Then
        assert_eq!(expected_exotic, exotic)
    }
}
