use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq)]
pub enum WeaponIcon {
    #[default]
    BlastersSelected,
    BlastersUnselected,
    TorpedoesSelected,
    TorpedoesUnselected,
    MinesSelected,
    MinesUnselected,
    ExoticsSelected,
    ExoticsUnselected,
}

impl Display for WeaponIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponIcon::BlastersSelected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/blasters_selected.png"
                )
            }
            WeaponIcon::BlastersUnselected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/blasters_unselected.png"
                )
            }
            WeaponIcon::TorpedoesSelected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/torpedoes_selected.png"
                )
            }
            WeaponIcon::TorpedoesUnselected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/torpedoes_unselected.png"
                )
            }
            WeaponIcon::MinesSelected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/mines_selected.png"
                )
            }
            WeaponIcon::MinesUnselected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/mines_unselected.png"
                )
            }
            WeaponIcon::ExoticsSelected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/exotics_selected.png"
                )
            }
            WeaponIcon::ExoticsUnselected => {
                write!(
                    formatter,
                    "images/user_interface/weapons/exotics_unselected.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod blaster_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/user_interface/weapons/blasters_selected.png";
        let blaster_sprite = WeaponIcon::default();

        // When
        let file_path = blaster_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        WeaponIcon::BlastersSelected,
        "images/user_interface/weapons/blasters_selected.png"
    )]
    #[case(
        WeaponIcon::BlastersUnselected,
        "images/user_interface/weapons/blasters_unselected.png"
    )]
    #[case(
        WeaponIcon::TorpedoesSelected,
        "images/user_interface/weapons/torpedoes_selected.png"
    )]
    #[case(
        WeaponIcon::TorpedoesUnselected,
        "images/user_interface/weapons/torpedoes_unselected.png"
    )]
    #[case(
        WeaponIcon::MinesSelected,
        "images/user_interface/weapons/mines_selected.png"
    )]
    #[case(
        WeaponIcon::MinesUnselected,
        "images/user_interface/weapons/mines_unselected.png"
    )]
    #[case(
        WeaponIcon::ExoticsSelected,
        "images/user_interface/weapons/exotics_selected.png"
    )]
    #[case(
        WeaponIcon::ExoticsUnselected,
        "images/user_interface/weapons/exotics_unselected.png"
    )]
    fn return_the_expected_file_path(
        #[case] weapon_selected: WeaponIcon,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = weapon_selected.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
