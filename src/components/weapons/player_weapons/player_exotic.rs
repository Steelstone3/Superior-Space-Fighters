use crate::components::weapons::ai_weapons::exotic::Exotic;
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct PlayerExotic {
    pub exotic: Exotic,
}

impl PlayerExotic {
    pub fn new(original_position: Vec3) -> Self {
        Self {
            exotic: Exotic::new(original_position),
        }
    }
}

#[cfg(test)]
mod player_exotic_should {
    use bevy::math::{Vec2, Vec3};

    use crate::components::weapons::weapon_types::{
        damage::Damage, ranged_weapon::RangedWeapon, weapon::Weapon,
    };

    use super::*;

    #[test]
    fn create_new() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_player_exotic = PlayerExotic {
            exotic: Exotic {
                exotic: Default::default(),
                firing_sound: Default::default(),
                impact_sound: Default::default(),
                ranged_weapon: RangedWeapon {
                    range: 500.0,
                    original_position,
                    weapon: Weapon {
                        velocity: 75.0,
                        size: Vec2 { x: 80.0, y: 80.0 },
                        damage: Damage { base_damage: 100 },
                    },
                },
            },
        };

        // When
        let exotic = PlayerExotic::new(original_position);

        // Then
        assert_eq!(expected_player_exotic, exotic)
    }
}
