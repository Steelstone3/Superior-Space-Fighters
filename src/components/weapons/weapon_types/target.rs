use crate::assets::images::starships::weapons::targetting::Targetting;
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Target {
    pub lock_on_target: Targetting,
    pub lock_on_target_size: Vec2,
    pub lock_on_target_off_screen: Targetting,
    pub lock_on_target_off_screen_size: Vec2,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            lock_on_target: Targetting::LockOnTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::LockOnTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        }
    }
}

#[cfg(test)]
mod target_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_targetting_settings = Target {
            lock_on_target: Targetting::LockOnTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::LockOnTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        };

        // When
        let targetting_settings = Target::default();

        // Then
        assert_eq!(expected_targetting_settings, targetting_settings);
    }
}
