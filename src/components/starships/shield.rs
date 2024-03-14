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
        if damage.damage >= self.current {
            self.current = 0;
            return;
        }

        self.current -= damage.damage;
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case(
        Damage {
            base_damage: 10,
            damage: 11,
        },
        Shield {
            maximum: 100,
            current: 89,
            regeneration: 5,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 20,
        },
        Shield {
            maximum: 100,
            current: 80,
            regeneration: 5,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 0,
        },
        Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 101,
        },
        Shield {
            maximum: 100,
            current: 0,
            regeneration: 5,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 100,
        },
        Shield {
            maximum: 100,
            current: 0,
            regeneration: 5,
    })]
    fn take_damage(#[case] damage: Damage, #[case] expected_shield: Shield) {
        // Given
        let mut shield = Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };

        // When
        shield.take_damage(damage);

        // Then
        assert_eq!(expected_shield, shield);
    }
}
