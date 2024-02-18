use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum MineSound {
    Mine1,
    Mine2,
    Mine3,
    Mine4,
    Mine5,
    Mine6,
}

impl Display for MineSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineSound::Mine1 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_1.ogg")
            }
            MineSound::Mine2 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_2.ogg")
            }
            MineSound::Mine3 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_3.ogg")
            }
            MineSound::Mine4 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_4.ogg")
            }
            MineSound::Mine5 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_5.ogg")
            }
            MineSound::Mine6 => {
                write!(formatter, "sounds/starships/weapons/mines/mine_6.ogg")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
