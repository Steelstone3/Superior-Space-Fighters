use bevy::ecs::component::Component;

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
}
