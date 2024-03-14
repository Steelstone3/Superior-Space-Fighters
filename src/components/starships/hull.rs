use bevy::ecs::component::Component;

use crate::components::weapons::weapon_types::damage::Damage;

// TODO implement regenerative hull when shield is 100
#[derive(Component, Debug, PartialEq)]
pub struct Hull {
    pub maximum: u32,
    pub current: u32,
    pub regeneration: u32,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            maximum: 100,
            current: 100,
            regeneration: 1,
        }
    }
}

impl Hull {
    pub fn take_damage(&mut self, damage: Damage) {
        if damage.damage >= self.current {
            self.current = 0;
            return;
        }

        self.current -= damage.damage;
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_new() {
        // Given
        let expected_hull = Hull {
            maximum: 100,
            current: 100,
            regeneration: 1,
        };

        // When
        let hull = Hull::default();

        // Then
        assert_eq!(expected_hull, hull);
    }

    #[rstest]
    #[case(
        Damage {
            base_damage: 10,
            damage: 0,
        },
        Hull {
            maximum: 100,
            current: 100,
            regeneration: 1,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 11,
        },
        Hull {
            maximum: 100,
            current: 89,
            regeneration: 1,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 100,
        },
        Hull {
            maximum: 100,
            current: 0,
            regeneration: 1,
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 101,
        },
        Hull {
            maximum: 100,
            current: 0,
            regeneration: 1,
    })]
    fn take_damage(#[case] damage: Damage, #[case] expected_hull: Hull) {
        // Given
        let mut hull = Hull {
            maximum: 100,
            current: 100,
            regeneration: 1,
        };

        // When
        hull.take_damage(damage);

        // Then
        assert_eq!(expected_hull, hull);
    }
}
