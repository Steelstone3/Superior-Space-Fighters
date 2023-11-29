use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum CombatMusicSound {
    AstralSovereignCombat,
    #[default]
    PirateCombat,
    SolarisUnionCombat,
}

impl Display for CombatMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatMusicSound::AstralSovereignCombat => {
                write!(formatter, "sounds/music/combat/astral_sovereign_combat.ogg")
            }
            CombatMusicSound::PirateCombat => {
                write!(formatter, "sounds/music/combat/pirate_combat.ogg")
            }
            CombatMusicSound::SolarisUnionCombat => {
                write!(formatter, "sounds/music/combat/solaris_union_combat.ogg")
            }
        }
    }
}
