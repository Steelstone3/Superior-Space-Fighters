use crate::assets::images::user_interface::weapon_selection::WeaponSelectedIcon;
use bevy::{ecs::component::Component, math::Vec2};

const SIZE: f32 = 1000.0;

#[derive(Component, Debug, PartialEq)]
pub struct WeaponIcon {
    pub size: Vec2,
    pub icon: WeaponSelectedIcon,
}

impl WeaponIcon {
    pub fn new_selected_blaster_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::BlastersSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_blaster_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::BlastersUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_selected_torpedo_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::TorpedoesSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_torpedo_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::TorpedoesUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_selected_mine_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::MinesSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_mine_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::MinesUnselected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_selected_exotic_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::ExoticsSelected,
            size: Vec2::new(SIZE, SIZE),
        }
    }
    pub fn new_unselected_exotic_icon() -> Self {
        Self {
            icon: WeaponSelectedIcon::ExoticsUnselected,
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
        let expected_weapon_icon = WeaponIcon {
            icon: WeaponSelectedIcon::BlastersSelected,
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
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
            size: Vec2::new(SIZE, SIZE),
        };

        // When
        let weapon_icon = WeaponIcon::new_unselected_exotic_icon();

        // Then
        assert_eq!(expected_weapon_icon, weapon_icon);
    }
}
