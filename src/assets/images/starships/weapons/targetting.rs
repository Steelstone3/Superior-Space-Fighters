use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen, Debug, PartialEq)]
pub enum Targetting {
    LockOnTarget,
    LockOnTargetOffScreen,
}

impl Display for Targetting {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Targetting::LockOnTarget => {
                write!(
                    formatter,
                    "images/starships/weapons/targetting/lock_on_target.png"
                )
            }
            Targetting::LockOnTargetOffScreen => {
                write!(
                    formatter,
                    "images/starships/weapons/targetting/lock_on_target_off_screen.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod targetting_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        Targetting::LockOnTarget,
        "images/starships/weapons/targetting/lock_on_target.png"
    )]
    #[case(
        Targetting::LockOnTargetOffScreen,
        "images/starships/weapons/targetting/lock_on_target_off_screen.png"
    )]
    fn return_the_expected_file_path(
        #[case] targetting_sprite: Targetting,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = targetting_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
