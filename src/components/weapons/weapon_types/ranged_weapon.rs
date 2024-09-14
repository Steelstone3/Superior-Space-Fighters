use super::weapon::Weapon;
use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::Vec3,
    reflect::Reflect,
};

#[derive(Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct RangedWeapon {
    pub range: f32,
    pub original_position: Vec3,
    pub weapon: Weapon,
}

impl RangedWeapon {
    pub fn new(original_position: Vec3, size: f32, velocity: f32, range: f32, damage: u32) -> Self {
        Self {
            range,
            original_position,
            weapon: Weapon::new(size, velocity, damage),
        }
    }
}

#[cfg(test)]
mod ranged_weapon_should {
    use super::*;
    use crate::components::weapons::weapon_types::damage::Damage;
    use bevy::math::Vec2;

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
        let base_damage = 10;
        let expected_ranged_weapon = RangedWeapon {
            range,
            original_position,
            weapon: Weapon {
                velocity,
                size: Vec2 { x: size, y: size },
                damage: Damage { base_damage },
            },
        };

        // When
        let ranged_weapon =
            RangedWeapon::new(original_position, size, velocity, range, base_damage);

        // Then
        assert_eq!(expected_ranged_weapon, ranged_weapon);
    }
}
