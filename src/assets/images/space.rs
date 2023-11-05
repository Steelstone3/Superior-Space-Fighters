use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum SpaceSprite {
    Space1,
    Space2,
    Space3,
    Space4,
    Space5,
    Space6,
}

impl Display for SpaceSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceSprite::Space1 => {
                write!(formatter, "images/background/space_1.png")
            }
            SpaceSprite::Space2 => {
                write!(formatter, "images/background/space_2.png")
            }
            SpaceSprite::Space3 => {
                write!(formatter, "images/background/space_3.png")
            }
            SpaceSprite::Space4 => {
                write!(formatter, "images/background/space_4.png")
            }
            SpaceSprite::Space5 => {
                write!(formatter, "images/background/space_5.png")
            }
            SpaceSprite::Space6 => {
                write!(formatter, "images/background/space_6.png")
            }
        }
    }
}
