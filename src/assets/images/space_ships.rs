use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum SpaceShipSprite {
    AlienFactionShip1,
    AlienFactionShip2,
    BlackFactionShip1,
    BlackFactionShip2,
    BlackFactionShip3,
    BlackFactionShip4,
    BlueFactionShip1,
    BlueFactionShip2,
    BlueFactionShip3,
    BlueFactionShip4,
    DeepBlueFactionShip1,
    DeepBlueFactionShip2,
    DeepBlueFactionShip3,
    DeepBlueFactionShip4,
    GreenFactionShip1,
    GreenFactionShip2,
    GreenFactionShip3,
    GreenFactionShip4,
    LightBlueFactionShip1,
    LightBlueFactionShip2,
    LightBlueFactionShip3,
    LightBlueFactionShip4,
    OrangeFactionShip1,
    OrangeFactionShip2,
    OrangeFactionShip3,
    OrangeFactionShip4,
    RedFactionShip1,
    RedFactionShip2,
    RedFactionShip3,
    RedFactionShip4,
    SteelFactionShip1,
    SteelFactionShip2,
    SteelFactionShip3,
    SteelFactionShip4,
}

impl Display for SpaceShipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceShipSprite::AlienFactionShip1 => {
                write!(formatter, "images/space_ships/alien_faction/ship_1.png")
            }
            SpaceShipSprite::AlienFactionShip2 => {
                write!(formatter, "images/space_ships/alien_faction/ship_2.png")
            }
            SpaceShipSprite::BlackFactionShip1 => {
                write!(formatter, "images/space_ships/black_faction/ship_1.png")
            }
            SpaceShipSprite::BlackFactionShip2 => {
                write!(formatter, "images/space_ships/black_faction/ship_2.png")
            }
            SpaceShipSprite::BlackFactionShip3 => {
                write!(formatter, "images/space_ships/black_faction/ship_3.png")
            }
            SpaceShipSprite::BlackFactionShip4 => {
                write!(formatter, "images/space_ships/black_faction/ship_4.png")
            }
            SpaceShipSprite::BlueFactionShip1 => {
                write!(formatter, "images/space_ships/blue_faction/ship_1.png")
            }
            SpaceShipSprite::BlueFactionShip2 => {
                write!(formatter, "images/space_ships/blue_faction/ship_2.png")
            }
            SpaceShipSprite::BlueFactionShip3 => {
                write!(formatter, "images/space_ships/blue_faction/ship_3.png")
            }
            SpaceShipSprite::BlueFactionShip4 => {
                write!(formatter, "images/space_ships/blue_faction/ship_4.png")
            }
            SpaceShipSprite::DeepBlueFactionShip1 => {
                write!(formatter, "images/space_ships/deep_blue_faction/ship_1.png")
            }
            SpaceShipSprite::DeepBlueFactionShip2 => {
                write!(formatter, "images/space_ships/deep_blue_faction/ship_2.png")
            }
            SpaceShipSprite::DeepBlueFactionShip3 => {
                write!(formatter, "images/space_ships/deep_blue_faction/ship_3.png")
            }
            SpaceShipSprite::DeepBlueFactionShip4 => {
                write!(formatter, "images/space_ships/deep_blue_faction/ship_4.png")
            }
            SpaceShipSprite::GreenFactionShip1 => {
                write!(formatter, "images/space_ships/green_faction/ship_1.png")
            }
            SpaceShipSprite::GreenFactionShip2 => {
                write!(formatter, "images/space_ships/green_faction/ship_2.png")
            }
            SpaceShipSprite::GreenFactionShip3 => {
                write!(formatter, "images/space_ships/green_faction/ship_3.png")
            }
            SpaceShipSprite::GreenFactionShip4 => {
                write!(formatter, "images/space_ships/green_faction/ship_4.png")
            }
            SpaceShipSprite::LightBlueFactionShip1 => {
                write!(
                    formatter,
                    "images/space_ships/light_blue_faction/ship_1.png"
                )
            }
            SpaceShipSprite::LightBlueFactionShip2 => {
                write!(
                    formatter,
                    "images/space_ships/light_blue_faction/ship_2.png"
                )
            }
            SpaceShipSprite::LightBlueFactionShip3 => {
                write!(
                    formatter,
                    "images/space_ships/light_blue_faction/ship_3.png"
                )
            }
            SpaceShipSprite::LightBlueFactionShip4 => {
                write!(
                    formatter,
                    "images/space_ships/light_blue_faction/ship_4.png"
                )
            }
            SpaceShipSprite::OrangeFactionShip1 => {
                write!(formatter, "images/space_ships/orange_faction/ship_1.png")
            }
            SpaceShipSprite::OrangeFactionShip2 => {
                write!(formatter, "images/space_ships/orange_faction/ship_2.png")
            }
            SpaceShipSprite::OrangeFactionShip3 => {
                write!(formatter, "images/space_ships/orange_faction/ship_3.png")
            }
            SpaceShipSprite::OrangeFactionShip4 => {
                write!(formatter, "images/space_ships/orange_faction/ship_4.png")
            }
            SpaceShipSprite::RedFactionShip1 => {
                write!(formatter, "images/space_ships/red_faction/ship_1.png")
            }
            SpaceShipSprite::RedFactionShip2 => {
                write!(formatter, "images/space_ships/red_faction/ship_2.png")
            }
            SpaceShipSprite::RedFactionShip3 => {
                write!(formatter, "images/space_ships/red_faction/ship_3.png")
            }
            SpaceShipSprite::RedFactionShip4 => {
                write!(formatter, "images/space_ships/red_faction/ship_4.png")
            }
            SpaceShipSprite::SteelFactionShip1 => {
                write!(formatter, "images/space_ships/steel_faction/ship_1.png")
            }
            SpaceShipSprite::SteelFactionShip2 => {
                write!(formatter, "images/space_ships/steel_faction/ship_2.png")
            }
            SpaceShipSprite::SteelFactionShip3 => {
                write!(formatter, "images/space_ships/steel_faction/ship_3.png")
            }
            SpaceShipSprite::SteelFactionShip4 => {
                write!(formatter, "images/space_ships/steel_faction/ship_4.png")
            }
        }
    }
}
