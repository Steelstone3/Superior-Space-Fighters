use crate::{
    components::weapons::player_weapons::player_exotic::PlayerExotic,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::{
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    selected_weapon: Res<SelectedWeapon>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

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

    let mut player_transform = *player_starship.transform;
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

    ammunition.exotic_ammunition -= 1;
    tracing::info!(
        "Fired 1 exotic shot. {:?} exotic shots remaining",
        ammunition.exotic_ammunition
    );
}
