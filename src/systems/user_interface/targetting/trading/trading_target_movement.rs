use crate::{
    queries::{
        filters::{
            player_starship_filters::PlayerStarshipWithoutStarshipAndTargetFilter,
            starship_filters::StarshipWithoutPlayerStarshipAndTargetFilter,
            target_filters::TargetWithoutPlayerStarshipAndStarshipFilter,
        },
        player_starship_queries::PlayerStarshipTransformQuery,
        starship_queries::StarshipTransformQuery,
        target_queries::MutableTargetTransformQuery,
    },
    resources::targetting_settings::TargettingSettings,
};
use bevy::{ecs::system::Res, prelude::Query};

pub fn trading_target_movement(
    targetting_setting: Res<TargettingSettings>,
    mut target_transforms: Query<
        MutableTargetTransformQuery,
        TargetWithoutPlayerStarshipAndStarshipFilter,
    >,
    player_starship_transforms: Query<
        PlayerStarshipTransformQuery,
        PlayerStarshipWithoutStarshipAndTargetFilter,
    >,
    starship_transforms: Query<
        StarshipTransformQuery,
        StarshipWithoutPlayerStarshipAndTargetFilter,
    >,
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
