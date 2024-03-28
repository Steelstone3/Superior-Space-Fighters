use crate::{
    components::weapons::player_weapons::player_mine::PlayerMine,
    queries::player_starship_queries::PlayerStarshipTransformQuery,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::{
    input::ButtonInput,
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_mine(
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

    if weapon_selection.selected_weapon != SelectedWeaponEnum::Mine as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.mine_ammunition < 1 {
        tracing::info!("Out of mines");
        return;
    }

    let mut player_transform = *player_starship.transform;
    let mine_size = 100.0;

    let mine_spawn_position =
        player_transform.translation + player_transform.down() * (mine_size / 1.75);
    player_transform.translation = mine_spawn_position;
    player_transform.translation.z = 3.0;

    let mine = PlayerMine::default();

    let image_path = mine.mine.mine.to_string();
    let sound_path = mine.mine.firing_sound.to_string();
    let texture = asset_server.load(image_path);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(mine.mine.lifetime_weapon.weapon.size),
                ..Default::default()
            },
            transform: player_transform,
            texture,
            ..Default::default()
        })
        .insert(mine);

    commands.spawn(AudioBundle {
        source: asset_server.load(sound_path),
        ..Default::default()
    });

    ammunition.mine_ammunition -= 1;
    tracing::info!(
        "Fired 1 mine. {:?} mines remaining",
        ammunition.mine_ammunition
    );
}
