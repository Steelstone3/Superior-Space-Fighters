use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    components::{mine::Mine, player::Player},
    resources::mine_ammunition::MineAmmunition,
};

pub fn spawn_mine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<MineAmmunition>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.get_single().unwrap();

    if ammunition.0 < 1.0 {
        info!("Out of mines");
        return;
    }

    let texture = asset_server.load("images/weapons/mines/mine_1.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: *player_transform,
            texture,
            ..Default::default()
        })
        .insert(Mine {
            speed: 0.0,
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        });

    ammunition.0 -= 1.0;
    info!("Fired 1 mine. {:?} mines remaining", ammunition.0);
}
