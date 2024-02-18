use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum SpaceStationSprite {
    AstralSovereignStation1,
    AstralSovereignStation2,
    AstralSovereignStation3,
    GeneralStation1,
    OuterReachMiningGuildStation1,
    SiliconFangTechnocracyStation1,
    SiliconFangTechnocracyStation2,
    SiliconFangTechnocracyStation3,
    SolarisUnionStation1,
}

impl Display for SpaceStationSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceStationSprite::AstralSovereignStation1 => {
                write!(
                    formatter,
                    "images/space_stations/astral_sovereign_station_1.png"
                )
            }
            SpaceStationSprite::AstralSovereignStation2 => {
                write!(
                    formatter,
                    "images/space_stations/astral_sovereign_station_2.png"
                )
            }
            SpaceStationSprite::AstralSovereignStation3 => {
                write!(
                    formatter,
                    "images/space_stations/astral_sovereign_station_3.png"
                )
            }
            SpaceStationSprite::GeneralStation1 => {
                write!(formatter, "images/space_stations/general_station_1.png")
            }
            SpaceStationSprite::OuterReachMiningGuildStation1 => {
                write!(
                    formatter,
                    "images/space_stations/outer_reach_mining_guild_station_1.png"
                )
            }
            SpaceStationSprite::SiliconFangTechnocracyStation1 => {
                write!(
                    formatter,
                    "images/space_stations/silicon_fang_technocracy_station_1.png"
                )
            }
            SpaceStationSprite::SiliconFangTechnocracyStation2 => {
                write!(
                    formatter,
                    "images/space_stations/silicon_fang_technocracy_station_2.png"
                )
            }
            SpaceStationSprite::SiliconFangTechnocracyStation3 => {
                write!(
                    formatter,
                    "images/space_stations/silicon_fang_technocracy_station_3.png"
                )
            }
            SpaceStationSprite::SolarisUnionStation1 => {
                write!(
                    formatter,
                    "images/space_stations/solaris_union_station_1.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
