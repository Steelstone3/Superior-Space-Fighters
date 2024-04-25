use super::weapon::Weapon;
use bevy::{
    ecs::component::Component,
    reflect::Reflect,
    time::{Timer, TimerMode},
};

#[derive(Component, Debug, PartialEq, Reflect)]
pub struct LifetimeWeapon {
    pub lifetime: Timer,
    pub weapon: Weapon,
}

impl LifetimeWeapon {
    pub fn new(size: f32, velocity: f32, base_damage: u32) -> Self {
        Self {
            weapon: Weapon::new(size, velocity, base_damage),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

#[cfg(test)]
mod lifetime_weapon_should {
    use bevy::math::Vec2;

    use crate::components::weapons::weapon_types::damage::Damage;

    use super::*;
    #[test]
    fn create_weapon() {
        // Given
        let velocity = 100.0;
        let size = 100.0;
        let base_damage = 10;
        let expected_lifetime_weapon = LifetimeWeapon {
            weapon: Weapon {
                velocity,
                size: Vec2 { x: size, y: size },
                damage: Damage { base_damage },
            },
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        // When
        let lifetime_weapon = LifetimeWeapon::new(size, velocity, base_damage);

        // Then
        assert_eq!(expected_lifetime_weapon, lifetime_weapon);
    }
}
