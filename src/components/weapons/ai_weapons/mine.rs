use crate::{
    assets::{
        images::starships::weapons::mines::MineSprite,
        sounds::starships::weapons::{impacts::ImpactSound, mines::MineSound},
    },
    components::weapons::weapon_types::lifetime_weapon::LifetimeWeapon,
};
use bevy::ecs::component::Component;

#[derive(Component, Debug, PartialEq)]
pub struct Mine {
    pub mine: MineSprite,
    pub firing_sound: MineSound,
    pub impact_sound: ImpactSound,
    pub lifetime_weapon: LifetimeWeapon,
}

impl Default for Mine {
    fn default() -> Self {
        Self {
            mine: MineSprite::default(),
            firing_sound: MineSound::default(),
            impact_sound: ImpactSound::default(),
            lifetime_weapon: LifetimeWeapon::new(100.0, -5.0),
        }
    }
}

#[cfg(test)]
mod mine_should {
    use crate::components::weapons::weapon_types::{damage::Damage, weapon::Weapon};

    use super::*;
    use bevy::{
        math::Vec2,
        time::{Timer, TimerMode},
    };

    #[test]
    fn create_new() {
        // Given
        let expected_mine = Mine {
            mine: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            lifetime_weapon: LifetimeWeapon {
                weapon: Weapon {
                    velocity: -5.0,
                    size: Vec2 { x: 100.0, y: 100.0 },
                    damage: Damage { base_damage: 10 },
                },
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            },
        };

        // When
        let mine = Mine::default();

        // Then
        assert_eq!(expected_mine, mine);
    }
}
