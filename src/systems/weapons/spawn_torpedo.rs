use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    assets::images::weapons::torpedos::TorpedoSprite,
    components::{player::Player, torpedo::Torpedo},
    resources::{selected_weapon::SelectedWeapon, torpedo_ammunition::TorpedoAmmunition},
};

pub fn spawn_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<TorpedoAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 2 {
        let player_transform = player.get_single().unwrap();

        if ammunition.0 < 1 {
            info!("Out of torpedos");
            return;
        }

        let torpedo = Torpedo {
            torpedo: TorpedoSprite::Torpedo1,
            speed: 0.0,
            size: Vec2::new(100.0, 100.0),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let texture = asset_server.load(torpedo.torpedo.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(torpedo.size),
                    ..Default::default()
                },
                transform: *player_transform,
                texture,
                ..Default::default()
            })
            .insert(torpedo);

        ammunition.0 -= 1;
        info!(
            "Fired 1 torpedo. {:?} torpedo ammunition remaining",
            ammunition.0
        );
    }
}
