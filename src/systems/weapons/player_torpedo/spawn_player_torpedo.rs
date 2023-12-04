use crate::{
    assets::{
        images::starships::weapons::torpedos::TorpedoSprite,
        sounds::starships::weapons::torpedos::TorpedoSound,
    },
    components::{
        player_starship::PlayerStarship, player_torpedo::PlayerTorpedo, torpedo::Torpedo,
    },
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    prelude::{
        AssetServer, AudioBundle, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2,
        With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
    utils::tracing,
};

pub fn spawn_player_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.selected_weapon == 2 {
        let mut player_transform = *player.get_single().unwrap();
        let torpedo_size = 80.0;

        let torpedo_spawn_position =
            player_transform.translation + player_transform.up() * (torpedo_size / 1.5);
        player_transform.translation = torpedo_spawn_position;
        player_transform.translation.z = 3.0;

        if ammunition.torpedo_ammunition < 1 {
            tracing::info!("Out of torpedos");
            return;
        }

        let torpedo = PlayerTorpedo {
            torpedo: Torpedo {
                torpedo: TorpedoSprite::Torpedo1,
                sound: TorpedoSound::Torpedo1,
                velocity: 125.0,
                size: Vec2::new(torpedo_size, torpedo_size),
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            },
        };

        let image_path = torpedo.torpedo.torpedo.to_string();
        let sound_path = torpedo.torpedo.sound.to_string();

        let texture = asset_server.load(image_path);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(torpedo.torpedo.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(torpedo);

        commands.spawn(AudioBundle {
            source: asset_server.load(sound_path),
            ..Default::default()
        });

        ammunition.torpedo_ammunition -= 1;
        tracing::info!(
            "Fired 1 torpedo. {:?} torpedo ammunition remaining",
            ammunition.torpedo_ammunition
        );
    }
}
