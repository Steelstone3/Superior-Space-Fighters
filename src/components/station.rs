use crate::assets::images::space_stations::SpaceStationSprite;
use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    prelude::Vec2,
    reflect::Reflect,
};

#[derive(Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SpaceStation {
    pub station: SpaceStationSprite,
    pub size: Vec2,
}

impl SpaceStation {
    pub fn new(station: SpaceStationSprite, size: f32) -> Self {
        Self {
            station,
            size: Vec2::new(size, size),
        }
    }
}

#[cfg(test)]
mod station_should {
    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_station = SpaceStation {
            station: SpaceStationSprite::AstralSovereignStation1,
            size: Vec2::new(100.0, 100.0),
        };

        // When
        let station = SpaceStation::new(SpaceStationSprite::AstralSovereignStation1, 100.0);

        // Then
        assert_eq!(expected_station, station)
    }
}
