use super::torpedo::Torpedo;
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct PlayerTorpedo {
    pub torpedo: Torpedo,
}

impl PlayerTorpedo {
    pub fn new(original_position: Vec3) -> Self {
        Self {
            torpedo: Torpedo::new(original_position),
        }
    }
}

#[cfg(test)]
mod player_torpedo_should {
    use crate::{
        assets::images::starships::weapons::targetting::Targetting,
        components::weapons::weapon_types::{
            damage::Damage, lock_on_weapon::LockOnWeapon, ranged_weapon::RangedWeapon,
            target::Target, weapon::Weapon,
        },
    };

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
        let expected_player_torpedo = PlayerTorpedo {
            torpedo: Torpedo {
                torpedo: Default::default(),
                firing_sound: Default::default(),
                impact_sound: Default::default(),
                lock_on_weapon: LockOnWeapon {
                    target: Target {
                        lock_on_target: Targetting::LockOnTarget,
                        lock_on_target_size: Vec2 { x: 100.0, y: 100.0 },
                        lock_on_target_off_screen: Targetting::LockOnTargetOffScreen,
                        lock_on_target_off_screen_size: Vec2 { x: 10.0, y: 10.0 },
                        starship_target: None,
                    },
                    ranged_weapon: RangedWeapon {
                        range: 1500.0,
                        original_position,
                        weapon: Weapon {
                            velocity: 125.0,
                            size: Vec2 { x: 80.0, y: 80.0 },
                            damage: Damage {
                                base_damage: 10,
                                damage: Default::default(),
                            },
                        },
                    },
                },
            },
        };

        // When
        let player_torpedo = PlayerTorpedo::new(original_position);

        // Then
        assert_eq!(expected_player_torpedo, player_torpedo)
    }
}
