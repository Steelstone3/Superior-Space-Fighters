use bevy::ecs::component::Component;

use super::starship::Starship;

#[derive(Component, Debug, PartialEq)]
pub struct PlayerStarship {
    pub ship: Starship,
    pub rotation: f32,
    pub acceleration: f32,
    pub current_velocity: f32,
}

impl Default for PlayerStarship {
    fn default() -> Self {
        Self {
            ship: Starship::default(),
            rotation: f32::to_radians(10.0),
            acceleration: 0.1,
            current_velocity: 0.0,
        }
    }
}

#[cfg(test)]
mod player_starship_should {
    use super::*;
    use crate::assets::images::starships::faction_starships::FactionStarshipSprite;
    use crate::assets::sounds::starships::engines::EngineSound;
    use bevy::math::Vec2;

    #[test]
    fn create_new() {
        // Given
        let expected_player_starship = PlayerStarship {
            ship: Starship {
                faction_starship: FactionStarshipSprite::OuterReachMiningGuildAllRounder,
                engine: EngineSound::Engine1,
                velocity: 30.0,
                size: Vec2 { x: 100.0, y: 100.0 },
            },
            rotation: f32::to_radians(10.0),
            acceleration: 0.1,
            current_velocity: 0.0,
        };

        // When
        let player_starship = PlayerStarship::default();

        // Then
        assert_eq!(expected_player_starship, player_starship);
    }
}
