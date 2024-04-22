use crate::{
    queries::{
        player_starship_queries::{PlayerStarshipFilter, PlayerStarshipTransformQuery},
        starship_queries::AIStarshipTransformQuery,
        target_queries::{TargetFilter, TargetMutableTransformQuery},
    },
    resources::targetting_settings::TargettingSettingsResource,
};
use bevy::{ecs::system::Res, prelude::Query};

pub fn combat_target_movement(
    targetting_setting: Res<TargettingSettingsResource>,
    mut target_transforms: Query<TargetMutableTransformQuery, TargetFilter>,
    player_starship_transforms: Query<PlayerStarshipTransformQuery, PlayerStarshipFilter>,
    starship_transforms: Query<AIStarshipTransformQuery>,
) {
    let Ok(player_starship_transform) = player_starship_transforms.get_single() else {
        return;
    };

    let Ok(mut target) = target_transforms.get_single_mut() else {
        return;
    };

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
            }
        }
    }

    // tracing::info!("Update Target Location");
    if let Some(target_starship) = closest_ship {
        *target.transform = *target_starship.transform;
    }
}
