use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum MenuMusicSound {
    #[default]
    Menu,
    Credits,
}

impl Display for MenuMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuMusicSound::Menu => {
                write!(formatter, "sounds/music/menu/menu.ogg")
            }
            MenuMusicSound::Credits => {
                write!(formatter, "sounds/music/menu/credits.ogg")
            }
        }
    }
}

#[cfg(test)]
mod menu_music_should {
    #[test]
    #[ignore]
    fn have_a_default() {}

    #[test]
    #[ignore]
    fn return_the_expected_file_path() {}
}
