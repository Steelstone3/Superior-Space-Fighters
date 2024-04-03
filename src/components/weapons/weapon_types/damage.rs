use bevy::ecs::component::Component;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Damage {
    pub base_damage: u32,
    pub damage: u32,
}

impl Default for Damage {
    fn default() -> Self {
        Self {
            base_damage: 10,
            damage: Default::default(),
        }
    }
}

impl Damage {
    //TODO implement a more complex damage system that randomly varies the damage
    pub fn calculate_damage(&mut self) {
        self.damage = self.base_damage + 1
    }
}

#[cfg(test)]
mod damage_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_damage = Damage {
            base_damage: 10,
            damage: Default::default(),
        };

        // When
        let damage = Damage::default();

        // Then
        assert_eq!(expected_damage, damage);
    }

    #[test]
    fn calculate_damage() {
        // Given
        let expected_damage = Damage {
            base_damage: 10,
            damage: 11,
        };
        let mut damage = Damage {
            base_damage: 10,
            damage: Default::default(),
        };

        // When
        damage.calculate_damage();

        // Then
        assert_eq!(expected_damage, damage);
    }
}
