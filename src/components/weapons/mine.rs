use crate::assets::{
    images::starships::weapons::mines::MineSprite,
    sounds::starships::weapons::{impacts::ImpactSound, mines::MineSound},
};
use bevy::{
    ecs::component::Component,
    math::Vec2,
    time::{Timer, TimerMode},
};

#[derive(Component, Debug, PartialEq)]
pub struct Mine {
    pub mine: MineSprite,
    pub firing_sound: MineSound,
    pub impact_sound: ImpactSound,
    pub velocity: f32,
    pub size: Vec2,
    pub lifetime: Timer,
}

impl Default for Mine {
    fn default() -> Self {
        Self {
            mine: MineSprite::default(),
            firing_sound: MineSound::default(),
            impact_sound: ImpactSound::default(),
            velocity: -5.0,
            size: Vec2::new(100.0, 100.0),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

#[cfg(test)]
mod mine_should {
    use super::*;
    use bevy::time::TimerMode;

    #[test]
    fn create_new() {
        // Given
        let expected_mine = Mine {
            mine: Default::default(),
            firing_sound: Default::default(),
            impact_sound: Default::default(),
            velocity: -5.0,
            size: Vec2 { x: 100.0, y: 100.0 },
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        // When
        let mine = Mine::default();

        // Then
        assert_eq!(expected_mine, mine);
    }
}
