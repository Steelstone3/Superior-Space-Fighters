use crate::components::{
    starships::starship::Starship,
    weapons::weapon_types::{target::Target, targetting_setting::TargettingSettings},
};
use bevy::{
    ecs::query::With,
    input::ButtonInput,
    prelude::{AssetServer, Commands, KeyCode, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_settings: Query<&mut TargettingSettings>,
    starship_transforms: Query<&Transform, With<Starship>>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(mut targetting_setting) = targetting_settings.get_single_mut() else {
        return;
    };

    let mut random_starship_transform = None;

    for starship_transform in starship_transforms.into_iter() {
        random_starship_transform = Some(starship_transform)
    }

    if let Some(random_starship) = random_starship_transform {
        if !targetting_setting.is_targetting {
            let target = Target::default();
            let texture = asset_server.load(target.lock_on_target.to_string());

            tracing::info!("Spawning Target");
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
