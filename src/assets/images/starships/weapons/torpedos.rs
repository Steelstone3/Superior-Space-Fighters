use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq)]
pub enum TorpedoSprite {
    #[default]
    Torpedo1,
    Torpedo2,
    Torpedo3,
    Torpedo4,
    Torpedo5,
    Torpedo6,
    Torpedo7,
    Torpedo8,
    Torpedo9,
}

impl Display for TorpedoSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorpedoSprite::Torpedo1 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_1.png")
            }
            TorpedoSprite::Torpedo2 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_2.png")
            }
            TorpedoSprite::Torpedo3 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_3.png")
            }
            TorpedoSprite::Torpedo4 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_4.png")
            }
            TorpedoSprite::Torpedo5 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_5.png")
            }
            TorpedoSprite::Torpedo6 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_6.png")
            }
            TorpedoSprite::Torpedo7 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_7.png")
            }
            TorpedoSprite::Torpedo8 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_8.png")
            }
            TorpedoSprite::Torpedo9 => {
                write!(formatter, "images/starships/weapons/torpedos/torpedo_9.png")
            }
        }
    }
}

#[cfg(test)]
mod torpedo_sprite_should {
    #[test]
    #[ignore]
    fn have_a_default() {}

    #[test]
    #[ignore]
    fn return_the_expected_file_path() {}
}
