use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
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
        player_transform.translation.z -= 1.0;
        // player_transform.translation.y -= 100.0;

        if ammunition.0 < 1 {
            info!("Out of mines");
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
        info!("Fired 1 mine. {:?} mines remaining", ammunition.0);
    }
}
