use bevy::{ecs::component::Component, math::Vec2};

use super::damage::Damage;

#[derive(Component, Debug, PartialEq)]
pub struct Weapon {
    pub velocity: f32,
    pub size: Vec2,
    pub damage: Damage,
}

impl Weapon {
    pub fn new(size: f32, velocity: f32, base_damage: u32) -> Self {
        Self {
            velocity,
            size: Vec2::new(size, size),
            damage: Damage::new(base_damage),
        }
    }
}

#[cfg(test)]
mod weapon_should {
    use super::*;
    #[test]
    fn create_weapon() {
        // Given
        let velocity = 100.0;
        let size = 100.0;
        let base_damage = 10;
        let expected_weapon = Weapon {
            velocity,
            size: Vec2 { x: size, y: size },
            damage: Damage { base_damage },
        };

        // When
        let weapon = Weapon::new(size, velocity, base_damage);

        // Then
        assert_eq!(expected_weapon, weapon);
    }
}
