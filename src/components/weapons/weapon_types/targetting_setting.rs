use bevy::ecs::component::Component;

#[derive(Component, Debug, PartialEq)]
pub struct TargettingSettings {
    pub maximum_distance: f32,
    pub is_targetting: bool,
}

impl Default for TargettingSettings {
    fn default() -> Self {
        Self {
            maximum_distance: 2000.0,
            is_targetting: false,
        }
    }
}

#[cfg(test)]
mod targetting_settings_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_targetting_settings = TargettingSettings {
            maximum_distance: 2000.0,
            is_targetting: false,
        };

        // When
        let targetting_settings = TargettingSettings::default();

        // Then
        assert_eq!(expected_targetting_settings, targetting_settings);
    }
}
