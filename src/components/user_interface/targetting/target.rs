use crate::assets::images::starships::weapons::targetting::Targetting;
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Target {
    pub lock_on_target: Targetting,
    pub lock_on_target_size: Vec2,
    pub lock_on_target_off_screen: Targetting,
    pub lock_on_target_off_screen_size: Vec2,
}

impl Target {
    pub fn create_combat_target() -> Self {
        Self {
            lock_on_target: Targetting::CombatTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::CombatTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        }
    }
    pub fn create_trading_target() -> Self {
        Self {
            lock_on_target: Targetting::TradingTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::TradingTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        }
    }
}

#[cfg(test)]
mod target_should {
    use super::*;

    #[test]
    fn create_new_combat_target() {
        // Given
        let expected_targetting_settings = Target {
            lock_on_target: Targetting::CombatTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::CombatTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        };

        // When
        let targetting_settings = Target::create_combat_target();

        // Then
        assert_eq!(expected_targetting_settings, targetting_settings);
    }

    #[test]
    fn create_new_trading_target() {
        // Given
        let expected_targetting_settings = Target {
            lock_on_target: Targetting::TradingTarget,
            lock_on_target_size: Vec2::new(100.0, 100.0),
            lock_on_target_off_screen: Targetting::TradingTargetOffScreen,
            lock_on_target_off_screen_size: Vec2::new(10.0, 10.0),
        };

        // When
        let targetting_settings = Target::create_trading_target();

        // Then
        assert_eq!(expected_targetting_settings, targetting_settings);
    }
}
