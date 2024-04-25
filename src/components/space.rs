use crate::assets::images::space::SpaceSprite;
use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::Vec3,
    prelude::Vec2,
    reflect::Reflect,
};

#[derive(Component, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct Space {
    pub space: SpaceSprite,
    pub size: Vec2,
    pub grid_position: Vec2,
    pub location: Vec3,
}

impl Space {
    pub fn new(space: SpaceSprite, size: f32, grid_position: Vec2, location: Vec3) -> Self {
        Self {
            space,
            size: Vec2::new(size, size),
            grid_position,
            location,
        }
    }
}

#[cfg(test)]
mod space_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_station = Space {
            space: SpaceSprite::Space1,
            size: Vec2::new(100.0, 100.0),
            grid_position: Vec2::new(1.0, 1.0),
            location: Vec3::new(2.0, 2.0, 2.0),
        };

        // When
        let station = Space::new(
            SpaceSprite::Space1,
            100.0,
            Vec2::new(1.0, 1.0),
            Vec3::new(2.0, 2.0, 2.0),
        );

        // Then
        assert_eq!(expected_station, station)
    }
}
