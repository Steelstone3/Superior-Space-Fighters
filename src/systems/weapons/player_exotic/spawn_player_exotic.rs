use bevy::{
    prelude::{
        AssetServer, AudioBundle, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2,
        With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
    utils::tracing,
};

use crate::{
    assets::{images::weapons::exotics::ExoticSprite, sounds::weapons::exotics::ExoticSound},
    components::{exotic::Exotic, player_exotic::PlayerExotic, player_starship::PlayerStarship},
    resources::{exotic_ammunition::ExoticAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<ExoticAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 4 {
        let mut player_transform = *player.get_single().unwrap();
        player_transform.translation.z = 3.0;
        let exotic_size = 80.0;

        if ammunition.0 < 1 {
            tracing::info!("Out of exotic ammunition");
            return;
        }

        let exotic = PlayerExotic {
            exotic: Exotic {
                exotic: ExoticSprite::Exotic1,
                sound: ExoticSound::Exotic1,
                velocity: 75.0,
                size: Vec2::new(exotic_size, exotic_size),
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            },
        };

        let image_path = exotic.exotic.exotic.to_string();
        let sound_path = exotic.exotic.sound.to_string();

        let texture = asset_server.load(image_path);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(exotic.exotic.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(exotic);

        commands.spawn(AudioBundle {
            source: asset_server.load(sound_path),
            ..Default::default()
        });

        ammunition.0 -= 1;
        tracing::info!(
            "Fired 1 exotic shot. {:?} exotic shots remaining",
            ammunition.0
        );
    }
}

#[cfg(test)]
mod spawn_player_exotic_should {
    use super::*;
    use bevy::{
        input::InputPlugin,
        prelude::{App, Update},
    };

    #[test]
    fn spawn_a_player_fired_exotic_projectile() {
        // Given
        let mut app = App::new();
        app.add_systems(Update, spawn_player_exotic);
        app.add_plugins(InputPlugin);
        // app.world
        //     .get_resource_mut::<Input<KeyCode>>()
        //     .unwrap()
        //     .just_pressed(KeyCode::W);

        // When
        // app.update();

        // Then
        // assert_eq!(app.world.query::<&PlayerExotic>().iter(&app.world).len(), 1);
    }
}
