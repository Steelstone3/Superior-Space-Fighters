use bevy::{
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    log,
    transform::components::Transform,
};

use crate::{
    components::starship::Starship,
    query_data::target_query::TargetQuery,
    query_filters::{player_starship_filter::PlayerStarshipFilter, target_filter::TargetFilter},
    resources::targeting_settings::TargetingSettings,
};

pub fn update_player_targeting(
    player_transform: Query<&Transform, PlayerStarshipFilter>,
    mut player_target: Query<TargetQuery, TargetFilter>,
    other_starships: Query<&mut Transform, With<Starship>>,
    mut commands: Commands,
    targeting_settings: Res<TargetingSettings>,
) {
    player_target.iter_mut().for_each(|mut target_query| {
        let Ok(player_transform) = player_transform.get_single() else {
            log::info!("player transform is not valid");
            return;
        };

        if let Ok(other_starship_transform) = other_starships.get(target_query.target.target_entity)
        {
            if (player_transform.translation - other_starship_transform.translation).length()
                <= targeting_settings.max_distance
            {
                target_query.transform.translation = other_starship_transform.translation;
                target_query.transform.rotation = other_starship_transform.rotation;
            } else {
                log::info!("target too far away despawning");
                commands.entity(target_query.entity).despawn();
            }
        } else {
            log::info!("target starship not found");
        }
    });
}
