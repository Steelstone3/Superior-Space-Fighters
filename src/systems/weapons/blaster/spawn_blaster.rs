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
    assets::{images::weapons::blasters::BlasterSprite, sounds::weapons::blasters::BlasterSound},
    components::{blaster::Blaster, player_starship::PlayerStarship},
    resources::{blaster_ammunition::BlasterAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_blaster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<BlasterAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player_query: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 1 {
        let mut player_transform = *player_query.get_single().unwrap();
        let blaster_size = 100.0;

        let blaster_spawn_position =
            player_transform.translation + player_transform.up() * (blaster_size / 2.0);
        player_transform.translation = blaster_spawn_position;
        player_transform.translation.z = 3.0;

        if ammunition.0 < 1 {
            tracing::info!("Out of blaster ammunition");
            return;
        }

        let blaster = Blaster {
            blaster: BlasterSprite::Blaster1,
            sound: BlasterSound::Blaster1,
            velocity: 100.0,
            size: Vec2::new(blaster_size, blaster_size),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let image_path = blaster.blaster.to_string();
        let sound_path = blaster.sound.to_string();

        let texture = asset_server.load(image_path);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(blaster.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(blaster);

        commands.spawn(AudioBundle {
            source: asset_server.load(sound_path),
            ..Default::default()
        });

        ammunition.0 -= 1;
        tracing::info!(
            "Fired 1 blaster shot. {:?} blaster shots remaining",
            ammunition.0
        );
    }
}
