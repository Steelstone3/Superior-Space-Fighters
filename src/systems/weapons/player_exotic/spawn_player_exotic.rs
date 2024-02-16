use crate::components::weapon::Weapon;
use crate::resources::projectile_ammunition::ProjectileAmmunition;
use crate::{
    assets::{
        images::starships::weapons::exotics::ExoticSprite,
        sounds::starships::weapons::exotics::ExoticSound,
    },
    components::{exotic::Exotic, player_exotic::PlayerExotic, player_starship::PlayerStarship},
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

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.selected_weapon == 4 {
        let mut player_transform = *player.get_single().unwrap();
        player_transform.translation.z = 3.0;
        let exotic_size = 80.0;

        if ammunition.exotic_ammunition < 1 {
            tracing::info!("Out of exotic ammunition");
            return;
        }

        let exotic = PlayerExotic {
            exotic: Exotic {
                exotic: ExoticSprite::Exotic1,
                sound: ExoticSound::Exotic1,
                weapon: Weapon {
                    size: Vec2::new(exotic_size, exotic_size),
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                    velocity: 75.0,
                },
            },
        };

        let image_path = exotic.exotic.exotic.to_string();
        let sound_path = exotic.exotic.sound.to_string();

        let texture = asset_server.load(image_path);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(exotic.exotic.weapon.size),
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

        ammunition.exotic_ammunition -= 1;
        tracing::info!(
            "Fired 1 exotic shot. {:?} exotic shots remaining",
            ammunition.exotic_ammunition
        );
    }
}
