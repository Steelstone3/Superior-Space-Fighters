use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    assets::images::weapons::blasters::BlasterSprite,
    components::{blaster::Blaster, player_starship::PlayerStarship},
    resources::{blaster_ammunition::BlasterAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_blaster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<BlasterAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 1 {
        let player_transform = *player.get_single().unwrap();
        // player_transform.translation.y += 100.0;

        if ammunition.0 < 1 {
            info!("Out of blaster ammunition");
            return;
        }

        let blaster = Blaster {
            blaster: BlasterSprite::Blaster1,
            velocity: 100.0,
            size: Vec2::new(100.0, 100.0),
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        };

        let texture = asset_server.load(blaster.blaster.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(blaster.size),
                    ..Default::default()
                },
                transform: player_transform,
                texture,
                ..Default::default()
            })
            .insert(blaster);

        ammunition.0 -= 1;
        info!(
            "Fired 1 blaster shot. {:?} blaster shots remaining",
            ammunition.0
        );
    }
}
