use super::{ranged_weapon::RangedWeapon, target::Target};
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct LockOnWeapon {
    pub target: Target,
    pub ranged_weapon: RangedWeapon,
}

impl LockOnWeapon {
    pub fn new(original_position: Vec3, size: f32, velocity: f32, range: f32) -> Self {
        Self {
            ranged_weapon: RangedWeapon::new(original_position, size, velocity, range),
            target: Target::default(),
        }
    }
}

#[cfg(test)]
mod lock_on_weapon_should {
    use super::*;
    use crate::{
        assets::images::starships::weapons::targetting::Targetting,
        components::weapons::weapon_types::{damage::Damage, weapon::Weapon},
    };
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
        let expected_lock_on_weapon = LockOnWeapon {
            ranged_weapon: RangedWeapon {
                range,
                original_position,
                weapon: Weapon {
                    velocity,
                    size: Vec2 { x: size, y: size },
                    damage: Damage {
                        base_damage: 10,
                        damage: Default::default(),
                    },
                },
            },
            target: Target {
                lock_on_target: Targetting::LockOnTarget,
                lock_on_target_size: Vec2 { x: 100.0, y: 100.0 },
                lock_on_target_off_screen: Targetting::LockOnTargetOffScreen,
                lock_on_target_off_screen_size: Vec2 { x: 10.0, y: 10.0 },
                starship_target: None,
            },
        };

        // When
        let lock_on_weapon = LockOnWeapon::new(original_position, size, velocity, range);

        // Then
        assert_eq!(expected_lock_on_weapon, lock_on_weapon);
    }
}
