use super::ranged_weapon::RangedWeapon;
use crate::assets::{
    images::starships::weapons::torpedos::TorpedoSprite,
    sounds::starships::weapons::{impacts::ImpactSound, torpedos::TorpedoSound},
};
use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug, PartialEq)]
pub struct Torpedo {
    pub torpedo: TorpedoSprite,
    pub firing_sound: TorpedoSound,
    pub impact_sound: ImpactSound,
    pub ranged_weapon: RangedWeapon,
}

impl Torpedo {
    pub fn new(original_position: Vec3) -> Self {
        Self {
            torpedo: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            ranged_weapon: RangedWeapon::new(original_position, 80.0, 125.0, 1500.0),
        }
    }
}

#[cfg(test)]
mod torpedo_should {
    use crate::components::weapons::{damage::Damage, ranged_weapon::RangedWeapon, weapon::Weapon};

    use super::*;
    use bevy::math::Vec2;

    #[test]
    fn create_new() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected_torpedo = Torpedo {
            torpedo: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            ranged_weapon: RangedWeapon {
                range: 1500.0,
                weapon: Weapon {
                    original_position,
                    velocity: 125.0,
                    size: Vec2 { x: 80.0, y: 80.0 },
                    damage: Damage {
                        base_damage: 10,
                        damage: Default::default(),
                    },
                },
            },
        };

        // When
        let torpedo = Torpedo::new(original_position);

        // Then
        assert_eq!(expected_torpedo, torpedo)
    }
}
