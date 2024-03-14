use bevy::ecs::component::Component;

use crate::components::weapons::damage::Damage;

use super::shield::Shield;

// TODO implement regenerative hull when shield is 100
#[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn take_damage(&mut self, _shield: Shield, _damage: Damage) {
        todo!()
    }
}

#[cfg(test)]
mod shield_should {
    use super::*;

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
}
