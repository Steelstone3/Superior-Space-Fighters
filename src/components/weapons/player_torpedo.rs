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
    use crate::components::weapons::weapon::Weapon;

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
                weapon: Weapon {
                    original_position,
                    velocity: 125.0,
                    size: Vec2 { x: 80.0, y: 80.0 },
                    range: 1500.0,
                },
            },
        };

        // When
        let player_torpedo = PlayerTorpedo::new(original_position);

        // Then
        assert_eq!(expected_player_torpedo, player_torpedo)
    }
}
