use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen, Debug, PartialEq)]
pub enum WeaponSelectedIcon {
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

impl Display for WeaponSelectedIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponSelectedIcon::BlastersSelected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/blaster_selected.png"
                )
            }
            WeaponSelectedIcon::BlastersUnselected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/blaster_unselected.png"
                )
            }
            WeaponSelectedIcon::TorpedoesSelected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/torpedoes_selected.png"
                )
            }
            WeaponSelectedIcon::TorpedoesUnselected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/torpedoes_unselected.png"
                )
            }
            WeaponSelectedIcon::MinesSelected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/mines_selected.png"
                )
            }
            WeaponSelectedIcon::MinesUnselected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/mines_unselected.png"
                )
            }
            WeaponSelectedIcon::ExoticsSelected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/exotics_selected.png"
                )
            }
            WeaponSelectedIcon::ExoticsUnselected => {
                write!(
                    formatter,
                    "images/user_interface/icons/weapons/exotics_unselected.png"
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
        let expected_file_path = "images/user_interface/icons/weapons/blaster_selected.png";
        let blaster_sprite = WeaponSelectedIcon::default();

        // When
        let file_path = blaster_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        WeaponSelectedIcon::BlastersSelected,
        "images/user_interface/icons/weapons/blaster_selected.png"
    )]
    #[case(
        WeaponSelectedIcon::BlastersUnselected,
        "images/user_interface/icons/weapons/blaster_unselected.png"
    )]
    #[case(
        WeaponSelectedIcon::TorpedoesSelected,
        "images/user_interface/icons/weapons/torpedoes_selected.png"
    )]
    #[case(
        WeaponSelectedIcon::TorpedoesUnselected,
        "images/user_interface/icons/weapons/torpedoes_unselected.png"
    )]
    #[case(
        WeaponSelectedIcon::MinesSelected,
        "images/user_interface/icons/weapons/mines_selected.png"
    )]
    #[case(
        WeaponSelectedIcon::MinesUnselected,
        "images/user_interface/icons/weapons/mines_unselected.png"
    )]
    #[case(
        WeaponSelectedIcon::ExoticsSelected,
        "images/user_interface/icons/weapons/exotics_selected.png"
    )]
    #[case(
        WeaponSelectedIcon::ExoticsUnselected,
        "images/user_interface/icons/weapons/exotics_unselected.png"
    )]
    fn return_the_expected_file_path(
        #[case] weapon_selected: WeaponSelectedIcon,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = weapon_selected.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
