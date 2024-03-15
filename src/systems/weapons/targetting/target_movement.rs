use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship, target_starship},
    weapons::weapon_types::{target::Target, targetting_setting::TargettingSettings},
};
use bevy::{
    prelude::{Commands, Query, Transform},
    utils::tracing,
};

pub fn target_movement(
    targetting_settings: Query<&TargettingSettings>,
    mut targets: Query<(&mut Transform, &Target)>,
    player_starships: Query<(&Transform, &PlayerStarship)>,
    starships: Query<(&Transform, &Starship)>,
) {
    let Ok(targetting_setting) = targetting_settings.get_single() else {
        return;
    };

    let Ok(mut target) = targets.get_single_mut() else {
        return;
    };

    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    // tracing::info!("Find Closest Starship",);

    let mut closest_ship = None;
    let mut distance = targetting_setting.maximum_distance;

    for starship in starships.into_iter() {
        let new_distance = (starship.0.translation - player_starship.0.translation).length();

        if new_distance < distance {
            distance = new_distance;

            if distance <= targetting_setting.maximum_distance {
                closest_ship = Some(starship);
                // tracing::info!("Closest Ship Found");
            }
        } else {
            continue;
        }
    }

    // tracing::info!("Update Target Location");
    match closest_ship {
        Some(target_starship) => {
            *target.0 = *target_starship.0;
        },
        None => todo!(),
    }
}
