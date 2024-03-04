use bevy::{
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    }, log, math::{Quat, Vec3}, render::{camera::{Camera, OrthographicProjection}, view::Visibility}, transform::components::{GlobalTransform, Transform}
};

use crate::{
    components::starship::Starship,
    query_data::{target_arrow_query::TargetArrowQuery, target_query::TargetQuery},
    query_filters::{
        player_starship_filter::PlayerStarshipFilter, target_arrow_filter::TargetArrowFilter,
        target_filter::TargetFilter,
    },
    resources::targeting_settings::TargetingSettings,
};

pub fn update_player_targeting(
    mut player_transform: Query<&Transform, PlayerStarshipFilter>,
    mut player_target: Query<TargetQuery, TargetFilter>,
    mut player_target_arrow: Query<TargetArrowQuery, TargetArrowFilter>,
    other_starships: Query<&mut Transform, With<Starship>>,
    mut commands: Commands,
    targeting_settings: Res<TargetingSettings>,
    camera: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let Ok(mut player_target) = player_target.get_single_mut() else {
        return;
    };

    let Ok(mut player_target_arrow) = player_target_arrow.get_single_mut() else {
        log::info!("player target arrow is not valid");
        return;
    };

    let Ok(player_transform) = player_transform.get_single_mut() else {
        log::info!("player transform is not valid");
        return;
    };

    if let Ok(other_starship_transform) = other_starships.get(player_target.target.target_entity) {
        //update target pos
        if (player_transform.translation - other_starship_transform.translation).length()
            <= targeting_settings.max_distance
        {
            let Ok(camera) = camera.get_single() else {
                return;
            };

            let camera_pos = camera.1.translation();
            let top = camera.2.area.max.y + camera_pos.y - 50.0;
            let right = camera.2.area.max.x + camera_pos.x - 50.0;
            let left = camera.2.area.min.x + camera_pos.x + 50.0;
            let bottom = camera.2.area.min.y + camera_pos.y + 50.0;

            player_target_arrow.transform.translation = other_starship_transform
                .translation
                .clamp(Vec3::new(left, bottom, 5.0), Vec3::new(right, top, 5.0));


            log::info!("{:?}",player_target_arrow.transform.translation.y);
            if player_target_arrow.transform.translation.x >= right || player_target_arrow.transform.translation.x <= left || player_target_arrow.transform.translation.y >= top || player_target_arrow.transform.translation.y <= bottom{
                *player_target_arrow.visible = Visibility::Visible;
            }
            else{
                *player_target_arrow.visible = Visibility::Hidden;

            }

            let pos = player_transform.translation.truncate();
            let target = other_starship_transform.translation.truncate();
            let angle = (pos - target).angle_between(pos);

            //due to 0,0 being the middle of screen need to offset the arrow angle depending on wether player is pos y or neg y otherwise arrow flips round when player goes down
            if player_transform.translation.y >= 0.0{
                player_target_arrow.transform.rotation = Quat::from_rotation_z(135.0 - angle);
            }else{
                player_target_arrow.transform.rotation = Quat::from_rotation_z(270.0 - angle);
            }

            player_target.transform.rotation = other_starship_transform.rotation;
            player_target.transform.translation = other_starship_transform.translation;
        } else {
            log::info!("target too far away despawning");
            commands.entity(player_target.entity).despawn();
            commands.entity(player_target_arrow.entity).despawn();
        }
    } else {
        log::info!("target starship not found");
    }
}
