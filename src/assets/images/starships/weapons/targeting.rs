use core::fmt::Display;

#[derive(Default, Debug, PartialEq)]
pub enum TargetingSprite {
    #[default]
    TargetArrow,
}

impl Display for TargetingSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetingSprite::TargetArrow => {
                write!(
                    formatter,
                    "images/user_interface/icons/targeting/up-arrow.png"
                )
            }
        }
    }
}

#[cfg(test)]
mod targeting_sprite_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn have_a_default() {
        // Given
        let expected_file_path = "images/user_interface/icons/targeting/up-arrow.png";
        let torpedo_sprite = TargetingSprite::default();

        // When
        let file_path = torpedo_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }

    #[rstest]
    #[case(
        TargetingSprite::TargetArrow,
        "images/user_interface/targeting/up-arrow.png"
    )]
    fn return_the_expected_file_path(
        #[case] torpedo_sprite: TargetingSprite,
        #[case] expected_file_path: String,
    ) {
        // When
        let file_path = torpedo_sprite.to_string();

        // Then
        assert_eq!(expected_file_path, file_path);
    }
}
