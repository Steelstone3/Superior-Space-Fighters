use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
    utils::tracing,
};

use crate::{
    assets::images::weapons::exotics::ExoticSprite,
    components::{exotic::Exotic, player_starship::PlayerStarship},
    resources::{exotic_ammunition::ExoticAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<ExoticAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 4 {
        let mut player_transform = *player.get_single().unwrap();
        let exotic_size = 80.0;

        let exotic_spawn_position =
            player_transform.translation + player_transform.up() * (exotic_size / 1.5);
        player_transform.translation = exotic_spawn_position;
        player_transform.translation.z = 3.0;

        if ammunition.0 < 1 {
            tracing::info!("Out of exotic ammunition");
            return;
        }

        let exotic = Exotic {
            exotic: ExoticSprite::Exotic1,
            velocity: 75.0,
            size: Vec2::new(exotic_size, exotic_size),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let texture = asset_server.load(exotic.exotic.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(exotic.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(exotic);

        ammunition.0 -= 1;
        tracing::info!(
            "Fired 1 exotic shot. {:?} exotic shots remaining",
            ammunition.0
        );
    }
}
