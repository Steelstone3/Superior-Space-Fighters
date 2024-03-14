use crate::{
    assets::{
        images::starships::faction_starships::FactionStarshipSprite,
        sounds::starships::engines::EngineSound,
    },
    components::weapons::damage::Damage,
};
use bevy::{ecs::component::Component, prelude::Vec2, utils::default};

use super::{hull::Hull, shield::Shield};

#[derive(Component, Debug, PartialEq)]
pub struct Starship {
    pub faction_starship: FactionStarshipSprite,
    pub engine: EngineSound,
    pub shield: Shield,
    pub hull: Hull,
    pub velocity: f32,
    pub size: Vec2,
}

impl Default for Starship {
    fn default() -> Self {
        Self {
            faction_starship: FactionStarshipSprite::default(),
            engine: EngineSound::default(),
            shield: Shield::default(),
            hull: Hull::default(),
            velocity: 30.0,
            size: Vec2::new(100.0, 100.0),
        }
    }
}

impl Starship {
    pub fn new(faction_starship: FactionStarshipSprite, engine: EngineSound) -> Self {
        Self {
            faction_starship,
            engine,
            shield: Shield::default(),
            hull: default(),
            velocity: 30.0,
            size: Vec2::new(100.0, 100.0),
        }
    }

    #[allow(dead_code)]
    pub fn take_damage(&mut self, _damage: Damage) {}
}

#[cfg(test)]
mod starship_should {
    use super::*;
    use crate::components::starships::hull::Hull;
    use crate::components::starships::shield::Shield;
    use rstest::rstest;

    #[test]
    fn create_new_player() {
        // Given
        let expected_starship = Starship {
            faction_starship: FactionStarshipSprite::OuterReachMiningGuildAllRounder,
            engine: EngineSound::Engine1,
            velocity: 30.0,
            size: Vec2 { x: 100.0, y: 100.0 },
            shield: Shield {
                maximum: 100,
                current: 100,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 100,
                regeneration: 1,
            },
        };

        // When
        let starship = Starship::default();

        // Then
        assert_eq!(expected_starship, starship);
    }

    #[test]
    fn create_new_ai() {
        // Given
        let expected_starship = Starship {
            faction_starship: FactionStarshipSprite::SiliconFangTechnocracyAllRounder,
            engine: EngineSound::Engine4,
            velocity: 30.0,
            size: Vec2 { x: 100.0, y: 100.0 },
            shield: Shield {
                maximum: 100,
                current: 100,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 100,
                regeneration: 1,
            },
        };

        // When
        let starship = Starship::new(
            FactionStarshipSprite::SiliconFangTechnocracyAllRounder,
            EngineSound::Engine4,
        );

        // Then
        assert_eq!(expected_starship, starship);
    }

    #[rstest]
    #[case(
        Damage {
            base_damage: 10,
            damage: 0,
        },
        Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        },
        Hull {
            maximum: 100,
            current: 100,
            regeneration: 5,
        })]
    fn take_damage(
        #[case] damage: Damage,
        #[case] expected_shield: Shield,
        #[case] expected_hull: Hull,
    ) {
        // Given
        let shield = Shield {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };
        let hull = Hull {
            maximum: 100,
            current: 100,
            regeneration: 5,
        };
        let mut starship = Starship {
            shield,
            hull,
            ..Default::default()
        };

        // When
        starship.take_damage(damage);

        // Then
        assert_eq!(expected_shield, starship.shield);
        assert_eq!(expected_hull, starship.hull);
    }
}
