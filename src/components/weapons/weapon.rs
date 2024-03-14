use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};

use super::damage::Damage;

#[derive(Component, Debug, PartialEq)]
pub struct Weapon {
    pub original_position: Vec3,
    pub velocity: f32,
    pub size: Vec2,
    pub damage: Damage,
}

impl Weapon {
    pub fn new(original_position: Vec3, size: f32, velocity: f32) -> Self {
        Self {
            original_position,
            velocity,
            size: Vec2::new(size, size),
            damage: Damage::default(),
        }
    }
}

#[cfg(test)]
mod weapon_should {
    use super::*;
    #[test]
    fn create_weapon() {
        // Given
        let original_position = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let velocity = 100.0;
        let size = 100.0;
        let expected_weapon = Weapon {
            original_position,
            velocity,
            size: Vec2 { x: size, y: size },
            damage: Damage {
                base_damage: 10,
                damage: Default::default(),
            },
        };

        // When
        let weapon = Weapon::new(original_position, size, velocity);

        // Then
        assert_eq!(expected_weapon, weapon);
    }
}
