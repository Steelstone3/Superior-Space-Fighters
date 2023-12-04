use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum BlasterSprite {
    Blaster1,
    Blaster2,
    Blaster3,
    Blaster4,
    Blaster5,
    Blaster6,
    Blaster7,
    Blaster8,
    Blaster9,
    Blaster10,
    Blaster11,
    Blaster12,
    Blaster13,
    Blaster14,
    Blaster15,
    Blaster16,
    Blaster17,
    Blaster18,
    Blaster19,
    Blaster20,
}

impl Display for BlasterSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlasterSprite::Blaster1 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_1.png")
            }
            BlasterSprite::Blaster2 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_2.png")
            }
            BlasterSprite::Blaster3 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_3.png")
            }
            BlasterSprite::Blaster4 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_4.png")
            }
            BlasterSprite::Blaster5 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_5.png")
            }
            BlasterSprite::Blaster6 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_6.png")
            }
            BlasterSprite::Blaster7 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_7.png")
            }
            BlasterSprite::Blaster8 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_8.png")
            }
            BlasterSprite::Blaster9 => {
                write!(formatter, "images/starships/weapons/blasters/blaster_9.png")
            }
            BlasterSprite::Blaster10 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_10.png"
                )
            }
            BlasterSprite::Blaster11 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_11.png"
                )
            }
            BlasterSprite::Blaster12 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_12.png"
                )
            }
            BlasterSprite::Blaster13 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_13.png"
                )
            }
            BlasterSprite::Blaster14 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_14.png"
                )
            }
            BlasterSprite::Blaster15 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_15.png"
                )
            }
            BlasterSprite::Blaster16 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_16.png"
                )
            }
            BlasterSprite::Blaster17 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_17.png"
                )
            }
            BlasterSprite::Blaster18 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_18.png"
                )
            }
            BlasterSprite::Blaster19 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_19.png"
                )
            }
            BlasterSprite::Blaster20 => {
                write!(
                    formatter,
                    "images/starships/weapons/blasters/blaster_20.png"
                )
            }
        }
    }
}
