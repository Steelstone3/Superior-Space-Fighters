use crate::{
    components::weapons::player_weapons::player_torpedo::PlayerTorpedo,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::{
    input::ButtonInput,
    math::Vec3,
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    weapon_selection: Res<SelectedWeapon>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

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
    let mut player_transform = *player_starship.transform;

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

    ammunition.torpedo_ammunition -= 1;
    tracing::info!(
        "Fired 1 torpedo. {:?} torpedo ammunition remaining",
        ammunition.torpedo_ammunition
    );
}
