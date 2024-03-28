use crate::queries::{
    camera_queries::{CameraFilter, CameraMutableTransformQuery},
    player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::ecs::system::Query;

pub fn camera_movement(
    player: Query<PlayerStarshipTransformQuery>,
    mut cameras: Query<CameraMutableTransformQuery, CameraFilter>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };

    camera.transform.translation = player.transform.translation;
}
