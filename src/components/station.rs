use crate::assets::images::space_stations::SpaceStationSprite;
use bevy::{ecs::component::Component, prelude::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Station {
    pub station: SpaceStationSprite,
    pub size: Vec2,
}

impl Station {
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
        let expected_station = Station {
            station: SpaceStationSprite::AstralSovereignStation1,
            size: Vec2::new(100.0, 100.0),
        };

        // When
        let station = Station::new(SpaceStationSprite::AstralSovereignStation1, 100.0);

        // Then
        assert_eq!(expected_station, station)
    }
}
