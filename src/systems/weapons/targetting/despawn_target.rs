use crate::components::{
    starships::starship::Starship,
    weapons::weapon_types::{target::Target, targetting_setting::TargettingSettings},
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

pub fn despawn_target(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_settings: Query<&mut TargettingSettings>,
    targets: Query<(Entity, &Target)>,
    starships: Query<&Starship>,
) {
    let Ok(mut targetting_setting) = targetting_settings.get_single_mut() else {
        return;
    };

    let Ok(target) = targets.get_single() else {
        return;
    };

    if input.just_pressed(KeyCode::Tab) {
        commands.entity(target.0).despawn();

        targetting_setting.is_targetting = false;

        tracing::info!("Despawning Target: Cancel Target Key Pressed");

        return;
    }

    let mut random_starship = None;

    for starship in starships.into_iter() {
        random_starship = Some(starship)
    }

    match random_starship {
        Some(_) => {}
        None => {
            commands.entity(target.0).despawn();
            targetting_setting.is_targetting = false;
            tracing::info!("Despawning Target: No Targets Exist");
        }
    }
}
