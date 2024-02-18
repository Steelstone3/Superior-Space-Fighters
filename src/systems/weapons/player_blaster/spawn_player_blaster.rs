use crate::{
    assets::{
        images::starships::weapons::blasters::BlasterSprite,
        sounds::starships::weapons::blasters::BlasterSound,
    },
    components::{
        blaster::Blaster, player_blaster::PlayerBlaster, player_starship::PlayerStarship,
        weapon::Weapon,
    },
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    input::ButtonInput,
    math::Vec3,
    prelude::{
        AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_blaster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    player_query: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.selected_weapon == 1 {
        let mut player_transform = *player_query.get_single().unwrap();
        player_transform.translation.z = 3.0;
        let blaster_size = 100.0;

        if ammunition.blaster_ammunition < 1 {
            tracing::info!("Out of blaster ammunition");
            return;
        }

        let blaster = PlayerBlaster {
            blaster: Blaster {
                blaster: BlasterSprite::Blaster1,
                sound: BlasterSound::Blaster1,
                weapon: Weapon {
                    original_position: Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        player_transform.translation.z,
                    ),
                    velocity: 100.0,
                    size: Vec2::new(blaster_size, blaster_size),
                    range: 750.0,
                },
            },
        };

        let image_path = blaster.blaster.blaster.to_string();
        let sound_path = blaster.blaster.sound.to_string();

        let texture = asset_server.load(image_path);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(blaster.blaster.weapon.size),
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

        ammunition.blaster_ammunition -= 1;
        tracing::info!(
            "Fired 1 blaster shot. {:?} blaster shots remaining",
            ammunition.blaster_ammunition
        );
    }
}
