use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum ExplorationMusicSound {
    AlienContact,
    #[default]
    DeepSpace1,
    DeepSpace2,
    DeepSpace3,
    NebulaExploration1,
    NebulaExploration2,
    NebulaExplorationScary,
}

impl Display for ExplorationMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplorationMusicSound::AlienContact => {
                write!(formatter, "sounds/music/exploration/alien_contact.ogg")
            }
            ExplorationMusicSound::DeepSpace1 => {
                write!(formatter, "sounds/music/exploration/deep_space_1.ogg")
            }
            ExplorationMusicSound::DeepSpace2 => {
                write!(formatter, "sounds/music/exploration/deep_space_2.ogg")
            }
            ExplorationMusicSound::DeepSpace3 => {
                write!(formatter, "sounds/music/exploration/deep_space_3.ogg")
            }
            ExplorationMusicSound::NebulaExploration1 => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_1.ogg"
                )
            }
            ExplorationMusicSound::NebulaExploration2 => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_2.ogg"
                )
            }
            ExplorationMusicSound::NebulaExplorationScary => {
                write!(
                    formatter,
                    "sounds/music/exploration/nebula_exploration_scary.ogg"
                )
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {}
