use crate::{
    components::{starships::starship::Starship, user_interface::targetting::target::Target},
    resources::targetting_settings::TargettingSettings,
};
use bevy::{
    ecs::{query::With, system::ResMut},
    input::ButtonInput,
    prelude::{AssetServer, Commands, KeyCode, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_trading_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_setting: ResMut<TargettingSettings>,
    starships: Query<&Transform, With<Starship>>,
) {
    if !input.just_pressed(KeyCode::KeyH) {
        return;
    }

    let mut random_starship_transform = None;

    for starship in starships.into_iter() {
        random_starship_transform = Some(starship)
    }

    if let Some(random_starship) = random_starship_transform {
        if !targetting_setting.is_targetting {
            let target = Target::create_trading_target();
            let texture = asset_server.load(target.lock_on_target.to_string());

            tracing::info!("Spawning Trading Target");
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(target.lock_on_target_size),
                        ..Default::default()
                    },
                    transform: *random_starship,
                    texture,
                    ..Default::default()
                })
                .insert(target);
        }
    }

    targetting_setting.is_targetting = true;
}
