use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(RandGen, Debug, PartialEq)]
pub enum Targetting {
    CombatTarget,
    TradingTarget,
    CombatTargetOffScreen,
    TradingTargetOffScreen,
}

impl Display for Targetting {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Targetting::CombatTarget => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_combat_target.png"
                )
            }
            Targetting::CombatTargetOffScreen => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_combat_target_off_screen_indicator.png"
                )
            }
            Targetting::TradingTarget => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_trading_target.png"
                )
            }
            Targetting::TradingTargetOffScreen => {
                write!(
                    formatter,
                    "images/user_interface/targetting/lock_on_trading_target_off_screen_indicator.png"
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
        Targetting::CombatTarget,
        "images/user_interface/targetting/lock_on_combat_target.png"
    )]
    #[case(
        Targetting::CombatTargetOffScreen,
        "images/user_interface/targetting/lock_on_combat_target_off_screen_indicator.png"
    )]
    #[case(
        Targetting::TradingTarget,
        "images/user_interface/targetting/lock_on_trading_target.png"
    )]
    #[case(
        Targetting::TradingTargetOffScreen,
        "images/user_interface/targetting/lock_on_trading_target_off_screen_indicator.png"
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
