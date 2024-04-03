use bevy::ecs::component::Component;

use crate::systems::controllers::random_generator::random_value_i32;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Damage {
    pub base_damage: u32,
}

impl Default for Damage {
    fn default() -> Self {
        Self { base_damage: 10 }
    }
}

impl Damage {
    pub fn calculate_damage(&self, seed: u64) -> u32 {
        (self.base_damage as i32 + random_value_i32(seed, -5..5)) as u32
    }
}

#[cfg(test)]
mod damage_should {
    use rstest::rstest;

    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_damage = Damage { base_damage: 10 };

        // When
        let damage = Damage::default();

        // Then
        assert_eq!(expected_damage, damage);
    }

    #[rstest]
    #[case(1234, 5)]
    #[case(4321, 12)]
    #[case(12344321, 7)]
    #[case(43211234, 7)]
    #[case(1111, 14)]
    fn calculate_damage(#[case] seed: u64, #[case] expected_damage: u32) {
        // Given
        let damage = Damage { base_damage: 10 };

        // When
        let damage = damage.calculate_damage(seed);

        // Then
        assert_eq!(expected_damage, damage);
    }
}
