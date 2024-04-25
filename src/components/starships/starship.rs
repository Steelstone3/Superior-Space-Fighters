use crate::assets::{
    images::starships::faction_starships::FactionStarshipSprite,
    sounds::starships::engines::EngineSound,
};
use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    prelude::Vec2,
    reflect::Reflect,
};

use super::{hull::Hull, shield::Shield};

#[derive(Component, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct Starship {
    pub faction_starship: FactionStarshipSprite,
    pub engine: EngineSound,
    pub shield: Shield,
    pub hull: Hull,
    pub acceleration: f32,
    pub max_velocity: f32,
    pub current_velocity: f32,
    pub rotation_speed: f32,
    pub size: Vec2,
}

impl Default for Starship {
    fn default() -> Self {
        Self {
            faction_starship: FactionStarshipSprite::default(),
            engine: EngineSound::default(),
            shield: Shield::default(),
            hull: Hull::default(),
            max_velocity: 30.0,
            acceleration: 0.1,
            current_velocity: 0.0,
            rotation_speed: 0.1,
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
            max_velocity: 30.0,
            acceleration: 0.1,
            current_velocity: 0.0,
            rotation_speed: 0.1,
            size: Vec2::new(100.0, 100.0),
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
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
            max_velocity: 30.0,
            acceleration: 0.1,
            current_velocity: 0.0,
            rotation_speed: 0.1,
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
            max_velocity: 30.0,
            acceleration: 0.1,
            current_velocity: 0.0,
            rotation_speed: 0.1,
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
        0,
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
        11,
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
        100,
        Starship{
            shield: Shield {
                maximum: 100,
                current: 0,
                regeneration: 5,
            },
            hull: Hull {
                maximum: 100,
                current: 50,
                regeneration: 1,
            },
            ..Default::default()
        })]
    #[case(
            101,
            Starship{
                shield: Shield {
                    maximum: 100,
                    current: 0,
                    regeneration: 5,
                },
                hull: Hull {
                    maximum: 100,
                    current: 50,
                    regeneration: 1,
                },
            ..Default::default()
    })]
    #[case(
        200,
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
    fn take_damage(#[case] damage: u32, #[case] expected_starship: Starship) {
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
