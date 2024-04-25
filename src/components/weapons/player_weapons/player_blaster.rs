use crate::components::weapons::ai_weapons::blaster::Blaster;
use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::Vec3,
    reflect::Reflect,
};

#[derive(Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct PlayerBlaster {
    pub blaster: Blaster,
}

impl PlayerBlaster {
    pub fn new(original_position: Vec3) -> Self {
        PlayerBlaster {
            blaster: Blaster::new(original_position),
        }
    }
}

#[cfg(test)]
mod player_blaster_should {
    use crate::components::weapons::weapon_types::{
        damage::Damage, ranged_weapon::RangedWeapon, weapon::Weapon,
    };

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
        let expected_player_blaster = PlayerBlaster {
            blaster: Blaster {
                blaster: Default::default(),
                firing_sound: Default::default(),
                impact_sound: Default::default(),
                ranged_weapon: RangedWeapon {
                    range: 750.0,
                    original_position,
                    weapon: Weapon {
                        velocity: 100.0,
                        size: Vec2 { x: 100.0, y: 100.0 },
                        damage: Damage { base_damage: 10 },
                    },
                },
            },
        };

        // When
        let player_blaster = PlayerBlaster::new(original_position);

        // Then
        assert_eq!(expected_player_blaster, player_blaster)
    }
}
