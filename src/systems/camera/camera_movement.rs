use crate::queries::{
    camera_queries::MutableCameraTransformQuery, filters::camera_filters::CameraWithoutPlayerStarshipFilter,
    player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::ecs::system::Query;

pub fn camera_movement(
    player: Query<PlayerStarshipTransformQuery>,
    mut camera: Query<MutableCameraTransformQuery, CameraWithoutPlayerStarshipFilter>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    camera.transform.translation = player.transform.translation;
}
