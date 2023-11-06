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
    Planet13,
    Planet14,
    Planet15,
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
    Planet78,
    Planet79,
    Planet80,
    Planet81,
    Planet82,
    Planet83,
    Planet84,
    Planet85,
    Planet86,
    Planet87,
    Planet88,
    Planet89,
    Planet90,
    Planet91,
    Planet92,
    Planet93,
    Planet94,
    Planet95,
    Planet96,
    Planet97,
    Planet98,
    Planet99,
    Planet100,
    Planet101,
    Planet102,
    Planet103,
    Planet104,
    Planet105,
    Planet106,
    Planet107,
    Planet108,
    Planet109,
    Planet110,
    Planet111,
    Planet112,
    Planet113,
    Planet114,
    Planet115,
    Planet156,
    Planet157,
    Planet158,
    Planet159,
    Planet160,
    Planet161,
    Planet162,
    Planet163,
    Planet164,
    Planet165,
    Planet166,
    Planet167,
    Planet168,
    Planet169,
    Planet170,
    Planet171,
    Planet172,
    Planet173,
    Planet174,
    Planet175,
    Planet176,
    Planet177,
    Planet178,
    Planet179,
    Planet180,
    Planet181,
    Planet182,
    Planet183,
    Planet184,
    Planet185,
    Planet186,
    Planet187,
    Planet188,
    Planet189,
    Planet190,
    Planet191,
    Planet192,
    Planet193,
    Planet194,
    Planet195,
    Planet196,
    Planet197,
    Planet198,
    Planet199,
    Planet200,
    Planet201,
    Planet202,
    Planet203,
    Planet204,
    Planet205,
    Planet206,
    Planet207,
    Planet208,
    Planet209,
    Planet210,
    Planet211,
    Planet212,
    Planet213,
    Planet214,
    Planet215,
    Planet216,
    Planet217,
    Planet218,
    Planet219,
    Planet220,
    Planet221,
    Planet222,
    Planet223,
    Planet224,
    Planet225,
    Planet226,
    Planet227,
    Planet228,
    Planet229,
    Planet230,
    Planet231,
    Planet232,
    Planet233,
    Planet234,
    Planet235,
    Planet236,
    Planet237,
    Planet238,
    Planet239,
    Planet240,
    Planet241,
    Planet242,
    Planet243,
    Planet244,
    Planet245,
    Planet246,
    Planet247,
    Planet248,
    Planet249,
}

impl Display for PlanetSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetSprite::Planet1 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet2 => {
                write!(formatter, "images/planets/planet_2.png")
            }
            PlanetSprite::Planet3 =>{write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet4 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet5 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet6 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet7 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet8 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet9 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet10 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet11 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet13 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet14 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet15 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet16 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet17 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet18 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet19 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet20 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet21 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet22 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet23 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet24 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet25 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet26 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet27 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet28 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet29 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet30 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet31 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet32 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet33 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet34 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet35 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet36 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet37 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet38 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet39 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet40 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet41 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet42 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet43 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet44 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet45 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet46 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet47 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet48 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet49 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet50 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet51 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet52 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet53 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet54 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet55 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet56 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet57 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet58 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet59 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet60 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet61 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet62 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet63 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet64 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet65 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet66 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet67 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet68 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet69 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet70 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet71 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet72 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet73 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet74 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet75 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet76 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet77 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet78 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet79 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet80 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet81 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet82 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet83 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet84 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet85 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet86 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet87 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet88 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet89 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet90 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet91 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet92 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet93 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet94 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet95 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet96 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet97 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet98 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet99 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet100 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet101 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet102 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet103 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet104 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet105 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet106 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet107 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet108 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet109 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet110 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet111 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet112 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet113 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet114 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet115 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet156 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet157 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet158 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet159 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet160 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet161 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet162 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet163 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet164 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet165 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet166 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet167 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet168 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet169 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet170 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet171 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet172 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet173 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet174 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet175 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet176 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet177 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet178 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet179 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet180 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet181 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet182 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet183 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet184 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet185 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet186 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet187 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet188 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet189 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet190 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet191 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet192 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet193 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet194 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet195 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet196 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet197 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet198 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet199 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet200 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet201 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet202 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet203 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet204 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet205 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet206 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet207 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet208 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet209 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet210 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet211 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet212 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet213 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet214 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet215 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet216 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet217 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet218 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet219 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet220 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet221 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet222 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet223 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet224 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet225 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet226 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet227 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet228 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet229 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet230 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet231 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet232 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet233 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet234 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet235 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet236 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet237 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet238 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet239 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet240 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet241 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet242 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet243 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet244 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet245 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet246 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet247 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet248 => {write!(formatter, "images/planets/planet_1.png")}
            PlanetSprite::Planet249 => {write!(formatter, "images/planets/planet_1.png")}
        }
    }
}
