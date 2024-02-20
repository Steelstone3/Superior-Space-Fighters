use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq)]
pub enum ExoticSound {
    #[default]
    Exotic1,
    Exotic2,
    Exotic3,
    Exotic4,
    Exotic5,
    Exotic6,
}

impl Display for ExoticSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExoticSound::Exotic1 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_1.ogg")
            }
            ExoticSound::Exotic2 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_2.ogg")
            }
            ExoticSound::Exotic3 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_3.ogg")
            }
            ExoticSound::Exotic4 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_4.ogg")
            }
            ExoticSound::Exotic5 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_5.ogg")
            }
            ExoticSound::Exotic6 => {
                write!(formatter, "sounds/starships/weapons/exotics/exotic_6.ogg")
            }
        }
    }
}

#[cfg(test)]
mod exotic_sound_should {
    #[test]
    #[ignore]
    fn have_a_default() {}

    #[test]
    #[ignore]
    fn return_the_expected_file_path() {}
}
