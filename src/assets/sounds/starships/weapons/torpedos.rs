use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum TorpedoSound {
    Torpedo1,
    Torpedo2,
    Torpedo3,
}

impl Display for TorpedoSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorpedoSound::Torpedo1 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_1.ogg")
            }
            TorpedoSound::Torpedo2 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_2.ogg")
            }
            TorpedoSound::Torpedo3 => {
                write!(formatter, "sounds/starships/weapons/torpedos/torpedo_3.ogg")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
