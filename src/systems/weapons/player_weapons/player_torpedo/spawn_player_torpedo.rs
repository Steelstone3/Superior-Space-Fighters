use std::time::Duration;

use crate::{
    components::{
        starships::player_starship::PlayerStarship,
        weapons::player_weapons::player_torpedo::PlayerTorpedo,
    },
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        projectile_fire_rate::ProjectileFireRate,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::{
    input::ButtonInput,
    math::Vec3,
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::tracing,
};

//TODO Find proper fix for this
#[allow(clippy::too_many_arguments)]
pub fn spawn_player_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    weapon_selection: Res<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
    mut weapon_fire_rate: ResMut<ProjectileFireRate>,
    time: Res<Time>,
) {
    //weapon still on cooldown
    if weapon_fire_rate.torpedo_fire_rate.remaining_secs() != 0.0 {
        weapon_fire_rate.torpedo_fire_rate.tick(time.delta());
        tracing::info!(
            "Torpedo time remaining {:?}",
            weapon_fire_rate.torpedo_fire_rate.remaining_secs()
        );
        return;
    }

    if weapon_selection.selected_weapon != SelectedWeaponEnum::Torpedo as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.torpedo_ammunition < 1 {
        tracing::info!("Out of torpedos");
        return;
    }

    let torpedo_size = 80.0;
    let mut player_transform = *player.get_single().unwrap();

    let torpedo_spawn_position =
        player_transform.translation + player_transform.up() * (torpedo_size / 1.5);
    player_transform.translation = torpedo_spawn_position;
    player_transform.translation.z = 3.0;

    let torpedo = PlayerTorpedo::new(Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ));

    let image_path = torpedo.torpedo.torpedo.to_string();
    let sound_path = torpedo.torpedo.firing_sound.to_string();

    let texture = asset_server.load(image_path);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(torpedo.torpedo.lock_on_weapon.ranged_weapon.weapon.size),
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

    //init cooldown duration
    if weapon_fire_rate.torpedo_fire_rate.duration() == Duration::from_secs(0) {
        weapon_fire_rate
            .torpedo_fire_rate
            .set_duration(Duration::from_secs(6));
    }

    ammunition.torpedo_ammunition -= 1;

    //reset cooldown
    weapon_fire_rate.torpedo_fire_rate.reset();

    tracing::info!(
        "Fired 1 torpedo. {:?} torpedo ammunition remaining",
        ammunition.torpedo_ammunition
    );
}
