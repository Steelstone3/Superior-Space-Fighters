use crate::components::player_starship::PlayerStarship;
use crate::components::weapons::player_exotic::PlayerExotic;
use crate::resources::projectile_ammunition::ProjectileAmmunition;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::{
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if ammunition.selected_weapon != 4 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.exotic_ammunition < 1 {
        tracing::info!("Out of exotic ammunition");
        return;
    }

    let mut player_transform = *player.get_single().unwrap();
    player_transform.translation.z = 3.0;

    let exotic = PlayerExotic::new(Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ));

    let image_path = exotic.exotic.exotic.to_string();
    let sound_path = exotic.exotic.firing_sound.to_string();
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
