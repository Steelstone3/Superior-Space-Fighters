use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};

#[derive(Component, Debug, PartialEq)]
pub struct Weapon {
    pub original_position: Vec3,
    pub velocity: f32,
    pub size: Vec2,
    pub range: f32,
}

impl Weapon {
    pub fn new(original_position: Vec3) -> Self {
        Weapon {
            original_position,
            velocity: 100.0,
            size: Vec2::new(100.0, 100.0),
            range: 750.0,
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
        let expected_weapon = Weapon {
            original_position,
            velocity: 100.0,
            size: Vec2 { x: 100.0, y: 100.0 },
            range: 750.0,
        };

        // When
        let weapon = Weapon::new(original_position);

        // Then
        assert_eq!(expected_weapon, weapon);
    }
}
