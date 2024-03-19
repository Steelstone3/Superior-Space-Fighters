use bevy::{ecs::component::Component, math::Vec2};

use crate::assets::images::user_interface::weapon_icons::WeaponIcon;

const SIZE: f32 = 1000.0;

#[derive(Component, Debug, PartialEq)]
pub struct WeaponSelection {
    pub size: Vec2,
    pub icon: WeaponIcon,
}

impl WeaponSelection {
    pub fn new_selected_blaster_icon() -> Self {
        Self {
            icon: WeaponIcon::BlastersSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    #[allow(dead_code)]
    pub fn new_unselected_blaster_icon() -> Self {
        Self {
            icon: WeaponIcon::BlastersUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    #[allow(dead_code)]
    pub fn new_selected_torpedo_icon() -> Self {
        Self {
            icon: WeaponIcon::TorpedoesSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_torpedo_icon() -> Self {
        Self {
            icon: WeaponIcon::TorpedoesUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    #[allow(dead_code)]
    pub fn new_selected_mine_icon() -> Self {
        Self {
            icon: WeaponIcon::MinesSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_mine_icon() -> Self {
        Self {
            icon: WeaponIcon::MinesUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    #[allow(dead_code)]
    pub fn new_selected_exotic_icon() -> Self {
        Self {
            icon: WeaponIcon::ExoticsSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_exotic_icon() -> Self {
        Self {
            icon: WeaponIcon::ExoticsUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
}

#[cfg(test)]
mod weapon_icon_should {
    use super::*;

    #[test]
    fn create_new_selected_blaster() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::BlastersSelected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_selected_blaster_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_blaster() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::BlastersUnselected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_unselected_blaster_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_torpedo() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::TorpedoesSelected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_selected_torpedo_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_torpedo() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::TorpedoesUnselected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_unselected_torpedo_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_mine() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::MinesSelected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_selected_mine_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_mine() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::MinesUnselected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_unselected_mine_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_selected_exotic() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::ExoticsSelected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_selected_exotic_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }

    #[test]
    fn create_new_unselected_exotic() {
        // Given
        let expected_weapon_icon = WeaponSelection {
            icon: WeaponIcon::ExoticsUnselected,
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponSelection::new_unselected_exotic_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }
}
