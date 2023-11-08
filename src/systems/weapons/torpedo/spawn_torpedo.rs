use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
    utils::tracing,
};

use crate::{
    assets::images::weapons::torpedos::TorpedoSprite,
    components::{player_starship::PlayerStarship, torpedo::Torpedo},
    resources::{selected_weapon::SelectedWeapon, torpedo_ammunition::TorpedoAmmunition},
};

pub fn spawn_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<TorpedoAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 2 {
        let mut player_transform = *player.get_single().unwrap();
        let torpedo_size = 80.0;

        let torpedo_spawn_position =
            player_transform.translation + player_transform.up() * (torpedo_size / 1.5);
        player_transform.translation = torpedo_spawn_position;
        player_transform.translation.z = 3.0;

        if ammunition.0 < 1 {
            tracing::info!("Out of torpedos");
            return;
        }

        let torpedo = Torpedo {
            torpedo: TorpedoSprite::Torpedo1,
            velocity: 125.0,
            size: Vec2::new(torpedo_size, torpedo_size),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let texture = asset_server.load(torpedo.torpedo.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(torpedo.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(torpedo);

        ammunition.0 -= 1;
        tracing::info!(
            "Fired 1 torpedo. {:?} torpedo ammunition remaining",
            ammunition.0
        );
    }
}
