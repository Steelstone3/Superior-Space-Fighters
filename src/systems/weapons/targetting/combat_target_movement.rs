use crate::{
    components::queries::{
        mutable_target_query::MutableTargetQuery, player_starship_filter::PlayerStarshipFilter,
        player_starship_query::PlayerStarshipQuery, starship_filter::StarshipFilter,
        starship_query::StarshipQuery, target_filter::TargetFilter,
    },
    resources::targetting_settings::TargettingSettings,
};
use bevy::{ecs::system::Res, prelude::Query};

pub fn combat_target_movement(
    targetting_setting: Res<TargettingSettings>,
    mut target_transforms: Query<MutableTargetQuery, TargetFilter>,
    player_starship_transforms: Query<PlayerStarshipQuery, PlayerStarshipFilter>,
    starship_transforms: Query<StarshipQuery, StarshipFilter>,
) {
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
        let new_distance = (starship_transform.transform.translation
            - player_starship_transform.transform.translation)
            .length();

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
        *target.transform = *target_starship.transform;
    }
}
