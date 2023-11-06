use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum PlanetSprite {
    Planet1,
    Planet2,
    Planet3,
    Planet4,
    Planet5,
    Planet6,
    Planet7,
    Planet8,
    Planet9,
    Planet10,
    Planet11,
    Planet13
    Planet14,
    Planet15
    Planet16,
    Planet17,
    Planet18,
    Planet19,
    Planet20,
    Planet21,
    Planet22,
    Planet23,
    Planet24,
    Planet25,
    Planet26,
    Planet27,
    Planet28,
    Planet29,
    Planet30,
    Planet31,
    Planet32,
    Planet33,
    Planet34,
    Planet35,
    Planet36,
    Planet37,
    Planet38,
    Planet39,
    Planet40,
    Planet41,
    Planet42,
    Planet43,
    Planet44,
    Planet45,
    Planet46,
    Planet47,
    Planet48,
    Planet49,
    Planet50,
    Planet51,
    Planet52,
    Planet53,
    Planet54,
    Planet55,
    Planet56,
    Planet57,
    Planet58,
    Planet59,
    Planet60,
    Planet61,
    Planet62,
    Planet63,
    Planet64,
    Planet65,
    Planet66,
    Planet67,
    Planet68,
    Planet69,
    Planet70,
    Planet71,
    Planet72,
    Planet73,
    Planet74,
    Planet75,
    Planet76,
    Planet77, 
}

impl Display for PlanetSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetSprite::Planet1 => {
                write!(formatter, "images/planets/planet_1.png")
            }
            PlanetSprite::Planet2 => {
                write!(formatter, "images/planets/planet_2.png")
            }
        }
    }
}
