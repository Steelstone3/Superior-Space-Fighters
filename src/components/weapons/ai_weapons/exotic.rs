use crate::{
    assets::{
        images::starships::weapons::exotics::ExoticSprite,
        sounds::starships::weapons::{exotics::ExoticSound, impacts::ImpactSound},
    },
    components::weapons::weapon_types::ranged_weapon::RangedWeapon,
};
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct Exotic {
    pub exotic: ExoticSprite,
    pub firing_sound: ExoticSound,
    pub impact_sound: ImpactSound,
    pub ranged_weapon: RangedWeapon,
}

impl Exotic {
    pub fn new(original_position: Vec3) -> Self {
        Self {
            exotic: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            ranged_weapon: RangedWeapon::new(original_position, 80.0, 75.0, 500.0),
        }
    }
}

#[cfg(test)]
mod exotic_should {
    use crate::components::weapons::weapon_types::{damage::Damage, weapon::Weapon};

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
            ranged_weapon: RangedWeapon {
                range: 500.0,
                original_position,
                weapon: Weapon {
                    velocity: 75.0,
                    size: Vec2 { x: 80.0, y: 80.0 },
                    damage: Damage { base_damage: 10 },
                },
            },
        };

        // When
        let exotic = Exotic::new(original_position);

        // Then
        assert_eq!(expected_exotic, exotic)
    }
}
