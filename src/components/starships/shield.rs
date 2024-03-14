use bevy::ecs::component::Component;

use crate::components::weapons::damage::Damage;

// TODO implement regenerative shields
#[allow(dead_code)]
#[derive(Component, Debug, PartialEq)]
pub struct Shield {
    pub maximum: u32,
    pub current: u32,
    pub regeneration: u32,
}

impl Default for Shield {
    fn default() -> Self {
        Self {
            maximum: 100,
            current: 100,
            regeneration: 5,
        }
    }
}

impl Shield {
    #[allow(dead_code)]
    pub fn take_damage(&mut self, damage: Damage) {
        self.current -= damage.calculated_damage;
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_shield = Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };

        // When
        let shield = Shield::default();

        // Then
        assert_eq!(expected_shield, shield);
    }

    #[test]
    fn take_damage() {
        // Given
        let expected_shield = Shield {
            maximum: 100,
            current: 89,
            regeneration: 5,
        };
        let mut shield = Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };
        let damage = Damage {
            base_damage: 10,
            calculated_damage: 11,
        };

        // When
        shield.take_damage(damage);

        // Then
        assert_eq!(expected_shield, shield);
    }
}
