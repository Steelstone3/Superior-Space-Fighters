use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
    utils::tracing,
};

use crate::{
    assets::images::weapons::mines::MineSprite,
    components::{mine::Mine, player_starship::PlayerStarship},
    resources::{mine_ammunition::MineAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_mine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<MineAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 3 {
        let mut player_transform = *player.get_single().unwrap();
        let mine_size = 100.0;

        let mine_spawn_position =
            player_transform.translation + player_transform.down() * (mine_size / 1.75);
        player_transform.translation = mine_spawn_position;
        player_transform.translation.z = 3.0;

        if ammunition.0 < 1 {
            tracing::info!("Out of mines");
            return;
        }

        let mine = Mine {
            mine: MineSprite::Mine1,
            velocity: -5.0,
            size: Vec2::new(100.0, 100.0),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let texture = asset_server.load(mine.mine.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(mine.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(mine);

        ammunition.0 -= 1;
        tracing::info!("Fired 1 mine. {:?} mines remaining", ammunition.0);
    }
}
