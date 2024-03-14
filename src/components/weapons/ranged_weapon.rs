use super::weapon::Weapon;
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct RangedWeapon {
    pub range: f32,
    pub weapon: Weapon,
}

impl RangedWeapon {
    pub fn new(original_position: Vec3, size: f32, velocity: f32, range: f32) -> Self {
        Self {
            range,
            weapon: Weapon::new(original_position, size, velocity),
        }
    }
}

#[cfg(test)]
mod weapon_should {
    use bevy::math::Vec2;

    use crate::components::weapons::damage::Damage;

    use super::*;
    #[test]
    fn create_weapon() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let velocity = 100.0;
        let size = 100.0;
        let range = 750.0;
        let expected_ranged_weapon = RangedWeapon {
            range,
            weapon: Weapon {
                original_position,
                velocity,
                size: Vec2 { x: size, y: size },
                damage: Damage {
                    base_damage: 10,
                    damage: Default::default(),
                },
            },
        };

        // When
        let ranged_weapon = RangedWeapon::new(original_position, size, velocity, range);

        // Then
        assert_eq!(expected_ranged_weapon, ranged_weapon);
    }
}
