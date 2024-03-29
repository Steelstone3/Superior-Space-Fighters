use bevy::ecs::component::Component;

// TODO implement regenerative shields
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
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        if damage >= self.current {
            self.current = 0;

            return damage / 2;
        }

        self.current -= damage;

        0
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
        11,
        0,
        Shield {
            maximum: 100,
            current: 89,
            regeneration: 5,
    })]
    #[case(
        20,
        0,
        Shield {
            maximum: 100,
            current: 80,
            regeneration: 5,
    })]
    #[case(
        0,
        0,
        Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
    })]
    #[case(
        101,
        50,
        Shield {
            maximum: 100,
            current: 0,
            regeneration: 5,
    })]
    #[case(
        200,
        100,
        Shield {
            maximum: 100,
            current: 0,
            regeneration: 5,
    })]
    fn take_damage(
        #[case] damage: u32,
        #[case] expected_damage: u32,
        #[case] expected_shield: Shield,
    ) {
        // Given
        let mut shield = Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };

        // When
        let damage = shield.take_damage(damage);

        // Then
        assert_eq!(expected_shield, shield);
        assert_eq!(expected_damage, damage);
    }
}
