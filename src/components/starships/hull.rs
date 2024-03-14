use bevy::ecs::component::Component;

#[allow(dead_code)]
#[derive(Component, Debug, PartialEq)]
pub struct Hull {
    pub maximum: u32,
    pub current: u32,
    pub regeneration: u32,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            maximum: 100,
            current: 100,
            regeneration: 1,
        }
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_hull = Hull {
            maximum: 100,
            current: 100,
            regeneration: 1,
        };

        // When
        let hull = Hull::default();

        // Then
        assert_eq!(expected_hull, hull);
    }
}
