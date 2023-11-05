use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
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
        player_transform.translation.z -= 1.0;
        // player_transform.translation.y += 100.0;

        if ammunition.0 < 1 {
            info!("Out of exotic ammunition");
            return;
        }

        let exotic = Exotic {
            exotic: ExoticSprite::Exotic1,
            velocity: 75.0,
            size: Vec2::new(80.0, 80.0),
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
        info!(
            "Fired 1 exotic shot. {:?} exotic shots remaining",
            ammunition.0
        );
    }
}
