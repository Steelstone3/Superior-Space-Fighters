use crate::{
    components::user_interface::targetting::target::Target,
    queries::starship_queries::StarshipTransformQuery,
    resources::targetting_settings::TargettingSettings,
};
use bevy::{
    ecs::system::ResMut,
    input::ButtonInput,
    prelude::{AssetServer, Commands, KeyCode, Query, Res},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_combat_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_setting: ResMut<TargettingSettings>,
    starships: Query<StarshipTransformQuery>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let mut random_starship_transform = None;

    for starship in starships.into_iter() {
        random_starship_transform = Some(starship.transform)
    }

    if let Some(random_starship) = random_starship_transform {
        if !targetting_setting.is_targetting {
            let target = Target::create_combat_target();
            let texture = asset_server.load(target.lock_on_target.to_string());

            tracing::info!("Spawning Combat Target");
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
