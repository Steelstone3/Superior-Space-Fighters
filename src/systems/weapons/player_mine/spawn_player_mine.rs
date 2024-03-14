use crate::{
    components::{starships::player_starship::PlayerStarship, weapons::player_mine::PlayerMine},
    resources::projectile_ammunition::ProjectileAmmunition,
};
use bevy::{
    input::ButtonInput,
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_player_mine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if ammunition.selected_weapon != 3 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.mine_ammunition < 1 {
        tracing::info!("Out of mines");
        return;
    }

    let mut player_transform = *player.get_single().unwrap();
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
