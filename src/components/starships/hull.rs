use bevy::{ecs::component::Component, reflect::Reflect};

// TODO implement regenerative hull when shield is 100
#[derive(Component, Debug, PartialEq, Reflect)]
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
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.current {
            self.current = 0;
            return;
        }

        self.current -= damage;
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
       0,
        Hull {
            maximum: 100,
            current: 100,
            regeneration: 1,
    })]
    #[case(
        11,
        Hull {
            maximum: 100,
            current: 89,
            regeneration: 1,
    })]
    #[case(
        100,
        Hull {
            maximum: 100,
            current: 0,
            regeneration: 1,
    })]
    #[case(
       101,
        Hull {
            maximum: 100,
            current: 0,
            regeneration: 1,
    })]
    fn take_damage(#[case] damage: u32, #[case] expected_hull: Hull) {
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
