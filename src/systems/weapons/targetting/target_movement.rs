use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    weapons::weapon_types::{target::Target, targetting_setting::TargettingSettings},
};
use bevy::{
    ecs::query::With,
    prelude::{Query, Transform},
};

#[allow(dead_code)]
pub fn target_movement(
    targetting_settings: Query<&TargettingSettings>,
    mut target_transforms: Query<&mut Transform, With<Target>>,
    player_starship_transforms: Query<&Transform, With<PlayerStarship>>,
    starship_transforms: Query<&Transform, With<Starship>>,
) {
    let Ok(targetting_setting) = targetting_settings.get_single() else {
        return;
    };

    let Ok(player_starship_transform) = player_starship_transforms.get_single() else {
        return;
    };

    let Ok(mut target) = target_transforms.get_single_mut() else {
        return;
    };

    // tracing::info!("Find Closest Starship",);

    let mut closest_ship = None;
    let mut distance = targetting_setting.maximum_distance;

    for starship_transform in starship_transforms.into_iter() {
        let new_distance =
            (starship_transform.translation - player_starship_transform.translation).length();

        if new_distance < distance {
            distance = new_distance;

            if distance <= targetting_setting.maximum_distance {
                closest_ship = Some(starship_transform);
                // tracing::info!("Closest Ship Found");
            }
        } else {
            continue;
        }
    }

    // tracing::info!("Update Target Location");
    if let Some(target_starship) = closest_ship {
        *target = *target_starship;
    }
}
