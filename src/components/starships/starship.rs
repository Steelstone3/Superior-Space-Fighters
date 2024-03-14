use crate::{
    assets::{
        images::starships::faction_starships::FactionStarshipSprite,
        sounds::starships::engines::EngineSound,
    },
    components::weapons::weapon_types::damage::Damage,
};
use bevy::{ecs::component::Component, prelude::Vec2};

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
            hull: Hull::default(),
            velocity: 30.0,
            size: Vec2::new(100.0, 100.0),
        }
    }

    pub fn take_damage(&mut self, damage: Damage) {
        let updated_damage = self.shield.take_damage(damage);
        self.hull.take_damage(updated_damage);
    }

    pub fn is_destroyed(&self) -> bool {
        self.hull.current == 0
    }
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
        Starship{
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
            ..Default::default()
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 11,
        },
        Starship{
            shield: Shield {
                maximum: 100,
                current: 89,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 100,
                regeneration: 1,
            },
            ..Default::default()
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 100,
        },
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 100,
                regeneration: 1,
            },
            ..Default::default()
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 101,
        },
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 99,
                regeneration: 1,
            },
            ..Default::default()
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 200,
        },
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 0,
                regeneration: 1,
            },
            ..Default::default()
    })]
    #[case(
        Damage {
            base_damage: 10,
            damage: 201,
        },
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 0,
                regeneration: 1,
            },
            ..Default::default()
    })]
    fn take_damage(#[case] damage: Damage, #[case] expected_starship: Starship) {
        // Given
        let mut starship = Starship {
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
            ..Default::default()
        };

        // When
        starship.take_damage(damage);

        // Then
        assert_eq!(expected_starship, starship);
    }

    #[rstest]
    #[case(
        Starship{
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
            ..Default::default()
        },
        false
    )]
    #[case(
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 100,
                regeneration: 1,
            },
            ..Default::default()
        },
        false
    )]
    #[case(
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 0,
                regeneration: 1,
            },
            ..Default::default()
        },
        true
    )]
    #[case(
        Starship{
            shield: Shield {
                maximum: 100,
                current: 10,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 0,
                regeneration: 1,
            },
            ..Default::default()
        },
        true
    )]
    fn is_destroyed(#[case] starship: Starship, #[case] expected_is_destroyed: bool) {
        // When
        let is_destoryed = starship.is_destroyed();

        // Then
        assert_eq!(expected_is_destroyed, is_destoryed);
    }
}
