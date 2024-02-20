use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq,)]
pub enum BlasterSound {
    #[default]
    Blaster1,
    Blaster2,
}

impl Display for BlasterSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlasterSound::Blaster1 => {
                write!(formatter, "sounds/starships/weapons/blasters/blaster_1.ogg")
            }
            BlasterSound::Blaster2 => {
                write!(formatter, "sounds/starships/weapons/blasters/blaster_2.ogg")
            }
        }
    }
}

#[cfg(test)]
mod blaster_sound_should {
    #[test]
    #[ignore]
    fn have_a_default() {}

    #[test]
    #[ignore]
    fn return_the_expected_file_path() {}
}
