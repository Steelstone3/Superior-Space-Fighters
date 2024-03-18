use crate::assets::images::user_interface::weapon_selected::WeaponSelectedIcon;
use bevy::ecs::component::Component;

#[derive(Component, Debug, PartialEq)]
pub struct WeaponIcon {
    pub icon: WeaponSelectedIcon,
}

impl WeaponIcon {
    pub fn new_selected_blaster_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::BlastersSelected,
        }
    }
    pub fn new_unselected_blaster_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::BlastersUnselected,
        }
    }
    pub fn new_selected_torpedo_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::TorpedoesSelected,
        }
    }
    pub fn new_unselected_torpedo_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::TorpedoesUnselected,
        }
    }
    pub fn new_selected_mine_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::MinesSelected,
        }
    }
    pub fn new_unselected_mine_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::MinesUnselected,
        }
    }
    pub fn new_selected_exotic_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::ExoticsSelected,
        }
    }
    pub fn new_unselected_exotic_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::ExoticsUnselected,
        }
    }
}

#[cfg(test)]
mod weapon_icon_should {
    use super::*;

    #[test]
    fn create_new_selected_blaster() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::BlastersSelected,
        };

        // When
        let weapon_icon = WeaponIcon::new_selected_blaster_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_blaster() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::BlastersUnselected,
        };

        // When
        let weapon_icon = WeaponIcon::new_unselected_blaster_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_torpedo() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::TorpedoesSelected,
        };

        // When
        let weapon_icon = WeaponIcon::new_selected_torpedo_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_torpedo() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::TorpedoesUnselected,
        };

        // When
        let weapon_icon = WeaponIcon::new_unselected_torpedo_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_mine() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::MinesSelected,
        };

        // When
        let weapon_icon = WeaponIcon::new_selected_mine_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_mine() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::MinesUnselected,
        };

        // When
        let weapon_icon = WeaponIcon::new_unselected_mine_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_exotic() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::ExoticsSelected,
        };

        // When
        let weapon_icon = WeaponIcon::new_selected_exotic_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_exotic() {
        // Given
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::ExoticsUnselected,
        };

        // When
        let weapon_icon = WeaponIcon::new_unselected_exotic_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }
}
