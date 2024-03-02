use bevy::{
    ecs::{
        query::{With, Without},
        system::{Commands, Query},
    },
    log,
    transform::components::Transform,
};

use crate::{
    components::{player_starship::PlayerStarship, starship::Starship},
    query_data::target_query::TargetQuery,
};

pub fn update_player_targeting(
    player_transform: Query<&Transform, (With<PlayerStarship>, Without<Starship>)>,
    mut player_target: Query<TargetQuery, (Without<Starship>, Without<PlayerStarship>)>,
    other_starships: Query<&mut Transform, With<Starship>>,
    mut commands: Commands,
) {
    player_target.iter_mut().for_each(|mut target_query| {
        let Ok(player_transform) = player_transform.get_single() else {
            log::info!("player transform is not valid");
            return;
        };

        if let Ok(other_starship_transform) = other_starships.get(target_query.target.target_entity)
        {
            if (player_transform.translation - other_starship_transform.translation).length()
                < 500.0
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
