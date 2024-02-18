use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum CombatMusicSound {
    AstralSovereign,
    #[default]
    Pirate,
    SolarisUnion,
}

impl Display for CombatMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatMusicSound::AstralSovereign => {
                write!(formatter, "sounds/music/combat/astral_sovereign_combat.ogg")
            }
            CombatMusicSound::Pirate => {
                write!(formatter, "sounds/music/combat/pirate_combat.ogg")
            }
            CombatMusicSound::SolarisUnion => {
                write!(formatter, "sounds/music/combat/solaris_union_combat.ogg")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
