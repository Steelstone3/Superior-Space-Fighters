use std::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum SpaceStationSprite {
    Station1,
    Station2,
    Station3,
    Station4,
    Station5,
    Station6,
    Station7,
    Station8,
    Station9,
}

impl Display for SpaceStationSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceStationSprite::Station1 => {
                write!(formatter, "images/space_stations/station_1.png")
            }
            SpaceStationSprite::Station2 => {
                write!(formatter, "images/space_stations/station_2.png")
            }
            SpaceStationSprite::Station3 => {
                write!(formatter, "images/space_stations/station_3.png")
            }
            SpaceStationSprite::Station4 => {
                write!(formatter, "images/space_stations/station_4.png")
            }
            SpaceStationSprite::Station5 => {
                write!(formatter, "images/space_stations/station_5.png")
            }
            SpaceStationSprite::Station6 => {
                write!(formatter, "images/space_stations/station_6.png")
            }
            SpaceStationSprite::Station7 => {
                write!(formatter, "images/space_stations/station_7.png")
            }
            SpaceStationSprite::Station8 => {
                write!(formatter, "images/space_stations/station_8.png")
            }
            SpaceStationSprite::Station9 => {
                write!(formatter, "images/space_stations/station_9.png")
            }
        }
    }
}
