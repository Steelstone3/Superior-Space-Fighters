use crate::{
    assets::{
        images::starships::faction_starships::FactionStarshipSprite,
        sounds::starships::engines::EngineSound,
    },
    components::weapons::damage::Damage,
};
use bevy::{ecs::component::Component, prelude::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Starship {
    pub faction_starship: FactionStarshipSprite,
    pub engine: EngineSound,
    pub velocity: f32,
    pub size: Vec2,
}

impl Default for Starship {
    fn default() -> Self {
        Self {
            faction_starship: FactionStarshipSprite::default(),
            engine: EngineSound::default(),
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
            velocity: 30.0,
            size: Vec2::new(100.0, 100.0),
        }
    }

    #[allow(dead_code)]
    pub fn take_damage(&mut self, _damage: Damage) {
        todo!()
    }
}

#[cfg(test)]
mod starship_should {
    use super::*;

    #[test]
    fn create_new_player() {
        // Given
        let expected_starship = Starship {
            faction_starship: FactionStarshipSprite::OuterReachMiningGuildAllRounder,
            engine: EngineSound::Engine1,
            velocity: 30.0,
            size: Vec2 { x: 100.0, y: 100.0 },
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
        };

        // When
        let starship = Starship::new(
            FactionStarshipSprite::SiliconFangTechnocracyAllRounder,
            EngineSound::Engine4,
        );

        // Then
        assert_eq!(expected_starship, starship);
    }
}
