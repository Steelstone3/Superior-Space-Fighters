use bevy::ecs::component::Component;

use crate::components::weapons::ai_weapons::mine::Mine;

#[derive(Component, Debug, PartialEq, Default)]
pub struct PlayerMine {
    pub mine: Mine,
}

#[cfg(test)]
mod player_mine_should {
    use crate::components::weapons::{
        ai_weapons::mine::Mine,
        weapon_types::{damage::Damage, lifetime_weapon::LifetimeWeapon, weapon::Weapon},
    };

    use super::*;
    use bevy::{
        math::Vec2,
        time::{Timer, TimerMode},
    };

    #[test]
    fn create_new() {
        // Given
        let expected_player_mine = PlayerMine {
            mine: Mine {
                mine: Default::default(),
                firing_sound: Default::default(),
                impact_sound: Default::default(),
                lifetime_weapon: LifetimeWeapon {
                    weapon: Weapon {
                        velocity: -5.0,
                        size: Vec2 { x: 100.0, y: 100.0 },
                        damage: Damage { base_damage: 75 },
                    },
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                },
            },
        };

        // When
        let mine = PlayerMine::default();

        // Then
        assert_eq!(expected_player_mine, mine);
    }
}
