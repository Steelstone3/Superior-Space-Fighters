use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum MineSprite {
    Mine1,
    Mine2,
    Mine3,
    Mine4,
    Mine5,
    Mine6,
    Mine7,
    Mine8,
    Mine9,
}

impl Display for MineSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineSprite::Mine1 => {
                write!(formatter, "images/starships/weapons/mines/mine_1.png")
            }
            MineSprite::Mine2 => {
                write!(formatter, "images/starships/weapons/mines/mine_2.png")
            }
            MineSprite::Mine3 => {
                write!(formatter, "images/starships/weapons/mines/mine_3.png")
            }
            MineSprite::Mine4 => {
                write!(formatter, "images/starships/weapons/mines/mine_4.png")
            }
            MineSprite::Mine5 => {
                write!(formatter, "images/starships/weapons/mines/mine_5.png")
            }
            MineSprite::Mine6 => {
                write!(formatter, "images/starships/weapons/mines/mine_6.png")
            }
            MineSprite::Mine7 => {
                write!(formatter, "images/starships/weapons/mines/mine_7.png")
            }
            MineSprite::Mine8 => {
                write!(formatter, "images/starships/weapons/mines/mine_8.png")
            }
            MineSprite::Mine9 => {
                write!(formatter, "images/starships/weapons/mines/mine_9.png")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
