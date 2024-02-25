use bevy::{ecs::component::Component, prelude::Vec2};

use crate::assets::images::planets::PlanetSprite;

#[derive(Component, Debug, PartialEq)]
pub struct Planet {
    pub planet: PlanetSprite,
    pub size: Vec2,
}

impl Planet {
    pub fn new(planet: PlanetSprite, size: f32) -> Self {
        Self {
            planet,
            size: Vec2::new(size, size),
        }
    }
}

#[cfg(test)]
mod planet_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_planet = Planet {
            planet: PlanetSprite::Planet1,
            size: Vec2::new(100.0, 100.0),
        };

        // When
        let planet = Planet::new(PlanetSprite::Planet1, 100.0);

        // Then
        assert_eq!(expected_planet, planet)
    }
}
