use std::time::Duration;

use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
    utils::tracing,
};

use crate::{
    components::{
        starships::player_starship::PlayerStarship,
        weapons::player_weapons::player_exotic::PlayerExotic,
    },
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        projectile_fire_rate::ProjectileFireRate,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    selected_weapon: Res<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
    mut weapon_fire_rate: ResMut<ProjectileFireRate>,
    time: Res<Time>,
) {
    //weapon still on cooldown
    if weapon_fire_rate.exotic_fire_rate.remaining_secs() != 0.0 {
        weapon_fire_rate.exotic_fire_rate.tick(time.delta());
        tracing::info!(
            "Exotic time remaining {:?}",
            weapon_fire_rate.exotic_fire_rate.remaining_secs()
        );
        return;
    }

    if selected_weapon.selected_weapon != SelectedWeaponEnum::Exotic as u32 {
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
                custom_size: Some(exotic.exotic.ranged_weapon.weapon.size),
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

    //init cooldown duration
    if weapon_fire_rate.exotic_fire_rate.duration() == Duration::from_secs(0) {
        weapon_fire_rate
            .exotic_fire_rate
            .set_duration(Duration::from_secs(8));
    }

    ammunition.exotic_ammunition -= 1;

    //reset cooldown
    weapon_fire_rate.exotic_fire_rate.reset();

    tracing::info!(
        "Fired 1 exotic shot. {:?} exotic shots remaining",
        ammunition.exotic_ammunition
    );
}
