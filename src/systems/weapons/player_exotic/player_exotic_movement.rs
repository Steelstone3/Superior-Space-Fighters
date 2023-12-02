use bevy::prelude::{Query, Res, Transform, Vec3};
use bevy::time::Time;

use crate::components::player_exotic::PlayerExotic;

pub fn player_exotic_movement(mut exotic: Query<(&mut Transform, &PlayerExotic)>, time: Res<Time>) {
    for (mut exotic_transform, exotic) in &mut exotic {
        let blaster_speed = exotic.exotic.velocity * time.delta_seconds();
        let movement_direction = exotic_transform.rotation * Vec3::Y;
        let translation_delta = movement_direction * blaster_speed;
        exotic_transform.translation += translation_delta;
    }
}

#[cfg(test)]
mod player_exotic_movement_should {
    use super::player_exotic_movement;
    use crate::{
        assets::{images::weapons::exotics::ExoticSprite, sounds::weapons::exotics::ExoticSound},
        components::{exotic::Exotic, player_exotic::PlayerExotic},
    };
    use bevy::{
        prelude::{App, Update, Vec2},
        time::{TimePlugin, Timer, TimerMode},
    };

    #[test]
    fn move_exotic_projectile() {
        // Given
        let mut app = App::new();
        app.add_systems(Update, player_exotic_movement);
        let exotic_id = app
            .world
            .spawn(PlayerExotic {
                exotic: Exotic {
                    exotic: ExoticSprite::Exotic1,
                    sound: ExoticSound::Exotic1,
                    velocity: 75.0,
                    size: Vec2::new(0.8, 0.8),
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                },
            })
            .id();
        app.add_plugins(TimePlugin);

        // When
        app.update();

        // Then
        assert!(app.world.get::<PlayerExotic>(exotic_id).is_some());
        // TODO test actual movement
    }
}
