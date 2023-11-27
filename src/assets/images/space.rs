use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Default)]
pub enum SpaceSprite {
    #[default]
    Space1,
    Space2,
    Space3,
    Space4,
    Space5,
    Space6,
    Space7,
    Space8,
    Space9,
    Space10,
    Space11,
    Space12,
    Space13,
    Space14,
    Space15,
    Space16,
    Space17,
    Space18,
    Space19,
    Space20,
    Space21,
    Space22,
    Space23,
    Space24,
    Space25,
    Space26,
    Space27,
    Space28,
    Space29,
    Space30,
    Space31,
    Space32,
}

impl Display for SpaceSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceSprite::Space1 => {
                write!(formatter, "images/background/Starfields/Starfield_01.png")
            },
            SpaceSprite::Space2 => {
                write!(formatter, "images/background/Starfields/Starfield_02.png")
            },
            SpaceSprite::Space3 => {
                write!(formatter, "images/background/Starfields/Starfield_03.png")
            },
            SpaceSprite::Space4 => {
                write!(formatter, "images/background/Starfields/Starfield_04.png")
            },
            SpaceSprite::Space5 => {
                write!(formatter, "images/background/Starfields/Starfield_05.png")
            },
            SpaceSprite::Space6 => {
                write!(formatter, "images/background/Starfields/Starfield_06.png")
            }
            SpaceSprite::Space7 => {
                write!(formatter, "images/background/Starfields/Starfield_07.png")
            }
            SpaceSprite::Space8 => {
                write!(formatter, "images/background/Starfields/Starfield_08.png")
            }
            SpaceSprite::Space9 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_01.png")
            },
            SpaceSprite::Space10 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_02.png")
            },
            SpaceSprite::Space11 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_03.png")
            },
            SpaceSprite::Space12 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_04.png")
            },
            SpaceSprite::Space13 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_05.png")
            },
            SpaceSprite::Space14 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_06.png")
            }
            SpaceSprite::Space15 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_07.png")
            }
            SpaceSprite::Space16 => {
                write!(formatter, "images/background/Purple Nebula/Purple_Nebula_08.png")
            }
            SpaceSprite::Space17 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space18 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space19 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space20 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space21 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space22 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space23 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space24 => {
                write!(formatter, "images/background/Green Nebula/Green_Nebula_08.png")
            },
            SpaceSprite::Space25 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_01.png")
            },
            SpaceSprite::Space26 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_02.png")
            },
            SpaceSprite::Space27 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_03.png")
            },
            SpaceSprite::Space28 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_04.png")
            },
            SpaceSprite::Space29 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_05.png")
            },
            SpaceSprite::Space30 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_06.png")
            },
            SpaceSprite::Space31 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_07.png")
            },
            SpaceSprite::Space32 => {
                write!(formatter, "images/background/Blue Nebula/Blue_Nebula_08.png")
            },
        }
    }
}
