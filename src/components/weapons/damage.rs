use bevy::ecs::component::Component;

#[derive(Component, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Damage {
    pub damage: u32,
}

impl Default for Damage {
    fn default() -> Self {
        Self { damage: 10 }
    }
}

#[cfg(test)]
mod damage_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_damage = Damage { damage: 10 };

        // When
        let damage = Damage::default();

        // Then
        assert_eq!(expected_damage, damage);
    }
}
